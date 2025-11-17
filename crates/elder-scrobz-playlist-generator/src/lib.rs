use anyhow::anyhow;
use elder_scrobz_db::{Period, PgPool, charts::tracks::get_most_listened_tracks};
use elder_scrobz_subsonic::SubsonicClient;

#[derive(Debug, Clone)]
pub struct PlaylistGenerator {
    pub client: SubsonicClient,
    pub db: PgPool,
}

impl PlaylistGenerator {
    pub async fn run(&self) -> anyhow::Result<()> {
        use tokio_cron_scheduler::{Job, JobScheduler};
        let sched = JobScheduler::new().await?;
        let periods = vec![
            (Period::Week, "0 0 * * 0"),  // Every Sunday
            (Period::Month, "0 0 L * *"), // Last day of the month
            (Period::Year, "0 0 1 1 *"),  // January 1st
        ];

        for (period, cron_expr) in periods {
            let period_clone = period.clone();
            let self_clone = self.clone();
            let job = Job::new_async(cron_expr, move |_uuid, _l| {
                let self_clone = self_clone.clone();
                Box::pin(async move {
                    self_clone
                        .generate_playlist(period_clone, None)
                        .await
                        .unwrap();
                })
            })?;
            sched.add(job).await?;
        }

        sched.start().await?;

        Ok(())
    }

    async fn generate_playlist(&self, period: Period, user: Option<&String>) -> anyhow::Result<()> {
        let (_, tracks) = get_most_listened_tracks(period, user, 100, 0, &self.db).await?;
        let subsonic_tracks: Vec<String> = tracks
            .into_iter()
            .filter_map(|track| track.subsonic_id)
            .collect();

        let period_str = match period {
            Period::Week => "of the week".to_string(),
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
}
