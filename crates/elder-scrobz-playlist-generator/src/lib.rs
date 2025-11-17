use anyhow::anyhow;
use elder_scrobz_db::{
    Period, PgPool,
    charts::tracks::get_most_listened_tracks,
    configs::{GlobalConfig, UserConfig},
};
use elder_scrobz_subsonic::SubsonicClient;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

#[derive(Clone)]
pub struct PlaylistGenerator {
    client: SubsonicClient,
    db: PgPool,
    scheduler: JobScheduler,
}

impl PlaylistGenerator {
    pub async fn create(
        client: SubsonicClient,
        db: PgPool,
    ) -> Result<Self, tokio_cron_scheduler::JobSchedulerError> {
        Ok(Self {
            client,
            db,
            scheduler: JobScheduler::new().await?,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("Starting playlist generator");
        let global = GlobalConfig::get(&self.db).await?;

        if global.enable_weekly_playlist {
            self.create_job(Period::Week, None).await?;
        }

        if global.enable_monthly_playlist {
            self.create_job(Period::Month, None).await?;
        }

        if global.enable_yearly_playlist {
            self.create_job(Period::Year, None).await?;
        }

        let user_configs = UserConfig::get_all(&self.db).await?;
        for user_config in user_configs {
            if user_config.enable_weekly_playlist {
                self.create_job(Period::Week, Some(user_config.username.clone()))
                    .await?;
            }

            if user_config.enable_monthly_playlist {
                self.create_job(Period::Month, Some(user_config.username.clone()))
                    .await?;
            }

            if user_config.enable_yearly_playlist {
                self.create_job(Period::Year, Some(user_config.username))
                    .await?;
            }
        }

        self.scheduler.set_shutdown_handler(Box::new(|| {
            Box::pin(async move {
                info!("Shut down done");
            })
        }));

        self.scheduler.start().await?;
        info!("scheduler OKKKK");

        Ok(())
    }

    async fn generate_playlist(&self, period: Period, user: Option<String>) -> anyhow::Result<()> {
        let (_, tracks) = get_most_listened_tracks(period, user.as_ref(), 100, 0, &self.db).await?;
        let subsonic_tracks: Vec<String> = tracks
            .into_iter()
            .filter_map(|track| track.subsonic_id)
            .collect();

        let period_str = match period {
            Period::Week => format!("of the week ({}", chrono::Utc::now().format("%d %B")),
            Period::Month => format!("of {}", chrono::Utc::now().format("%B")),
            Period::Year => "of the year".to_string(),
            _ => return Err(anyhow!("Could not generate playlist for this time range")),
        };

        let user = user.map(|u| format!("{u}'s"));
        let playlist_name = format!(
            "{}'s mix {}",
            user.as_deref().unwrap_or("Global"),
            period_str
        );

        if !subsonic_tracks.is_empty() {
            self.client
                .create_playlist(&playlist_name, subsonic_tracks)
                .await?;
        }

        Ok(())
    }

    async fn create_job(
        &self,
        period: Period,
        user: Option<String>,
    ) -> Result<(), tokio_cron_scheduler::JobSchedulerError> {
        let this = self.clone();
        let job = Job::new_async(period_to_cron_expr(period), move |_uuid, _l| {
            let user = user.clone();
            let this = this.clone();
            Box::pin(async move {
                this.generate_playlist(period, user.clone()).await.unwrap();
            })
        })?;

        job.on_start_notification_add(
            &self.scheduler,
            Box::new(|job_id, notification_id, type_of_notification| {
                Box::pin(async move {
                    info!(
                        "Job {:?} was started, notification {:?} ran ({:?})",
                        job_id, notification_id, type_of_notification
                    );
                })
            }),
        )
        .await?;

        self.scheduler.add(job).await?;
        Ok(())
    }
}

fn period_to_cron_expr(period: Period) -> String {
    match period {
        Period::Week => "0 0 23 * * Sun".to_string(),
        Period::Month => "0 0 0 L * *".to_string(),
        Period::Year => "0 0 0 31 12 *".to_string(),
        _ => panic!("Unsupported period"),
    }
}
