import {
	MusicStats,
	TimeRange,
	RecentTrack,
	ArtistDetails,
	ApiKey,
	User,
	AlbumDetails,
	PulseTimeRange,
	PulseData,
} from '@/types/music';
import type { Artist, Album, Track } from '@/types/music';

export const mockArtists: Artist[] = [
	{
		id: '1',
		name: 'The Elder Scrolls',
		imageUrl: 'https://picsum.photos/200/200?random=1',
		playCount: 1234,
		duration: 45678,
	},
	{
		id: '2',
		name: 'Pink Floyd',
		imageUrl: 'https://picsum.photos/200/200?random=2',
		playCount: 120,
		duration: 270,
	},
	{
		id: '3',
		name: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=3',
		playCount: 100,
		duration: 300,
	},
	{
		id: '4',
		name: 'David Bowie',
		imageUrl: 'https://picsum.photos/200/200?random=4',
		playCount: 90,
		duration: 240,
	},
	{
		id: '5',
		name: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=5',
		playCount: 85,
		duration: 450,
	},
	{
		id: '6',
		name: 'The Rolling Stones',
		imageUrl: 'https://picsum.photos/200/200?random=6',
		playCount: 80,
		duration: 270,
	},
	{
		id: '7',
		name: 'Nirvana',
		imageUrl: 'https://picsum.photos/200/200?random=7',
		playCount: 75,
		duration: 300,
	},
	{
		id: '8',
		name: 'Radiohead',
		imageUrl: 'https://picsum.photos/200/200?random=8',
		playCount: 70,
		duration: 270,
	},
	{
		id: '9',
		name: 'Arctic Monkeys',
		imageUrl: 'https://picsum.photos/200/200?random=9',
		playCount: 65,
		duration: 300,
	},
	{
		id: '10',
		name: 'Tame Impala',
		imageUrl: 'https://picsum.photos/200/200?random=10',
		playCount: 60,
		duration: 270,
	},
	{
		id: '11',
		name: 'Eagles',
		imageUrl: 'https://picsum.photos/200/200?random=11',
		playCount: 55,
		duration: 300,
	},
	{
		id: '12',
		name: 'Neil Diamond',
		imageUrl: 'https://picsum.photos/200/200?random=12',
		playCount: 50,
		duration: 270,
	},
	{
		id: '13',
		name: 'The Beatles',
		imageUrl: 'https://picsum.photos/200/200?random=13',
		playCount: 45,
		duration: 300,
	},
	{
		id: '14',
		name: 'The Beach Boys',
		imageUrl: 'https://picsum.photos/200/200?random=14',
		playCount: 40,
		duration: 270,
	},
	{
		id: '15',
		name: 'The Who',
		imageUrl: 'https://picsum.photos/200/200?random=15',
		playCount: 35,
		duration: 300,
	},
];

export const mockAlbums: Album[] = [
	{
		id: '1',
		title: 'Skyrim Soundtrack',
		artist: 'The Elder Scrolls',
		imageUrl: 'https://picsum.photos/200/200?random=2',
		playCount: 567,
		duration: 12345,
	},
	{
		id: '2',
		title: 'The Wall',
		artist: 'Pink Floyd',
		imageUrl: 'https://picsum.photos/200/200?random=22',
		playCount: 70,
		duration: 42.9,
	},
	{
		id: '3',
		title: 'Led Zeppelin IV',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=23',
		playCount: 60,
		duration: 42.3,
	},
	{
		id: '4',
		title: 'David Bowie',
		artist: 'David Bowie',
		imageUrl: 'https://picsum.photos/200/200?random=24',
		playCount: 50,
		duration: 38.2,
	},
	{
		id: '5',
		title: 'A Night at the Opera',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=25',
		playCount: 45,
		duration: 43.0,
	},
	{
		id: '6',
		title: 'Aftermath',
		artist: 'The Rolling Stones',
		imageUrl: 'https://picsum.photos/200/200?random=26',
		playCount: 40,
		duration: 42.9,
	},
	{
		id: '7',
		title: 'Nevermind',
		artist: 'Nirvana',
		imageUrl: 'https://picsum.photos/200/200?random=27',
		playCount: 35,
		duration: 42.3,
	},
	{
		id: '8',
		title: 'Pablo Honey',
		artist: 'Radiohead',
		imageUrl: 'https://picsum.photos/200/200?random=28',
		playCount: 30,
		duration: 42.9,
	},
	{
		id: '9',
		title: 'AM',
		artist: 'Arctic Monkeys',
		imageUrl: 'https://picsum.photos/200/200?random=29',
		playCount: 25,
		duration: 47.2,
	},
	{
		id: '10',
		title: 'Currents',
		artist: 'Tame Impala',
		imageUrl: 'https://picsum.photos/200/200?random=30',
		playCount: 20,
		duration: 42.9,
	},
	{
		id: '11',
		title: 'Hotel California',
		artist: 'Eagles',
		imageUrl: 'https://picsum.photos/200/200?random=31',
		playCount: 15,
		duration: 42.9,
	},
	{
		id: '12',
		title: 'The Dark Side of the Moon',
		artist: 'Pink Floyd',
		imageUrl: 'https://picsum.photos/200/200?random=32',
		playCount: 10,
		duration: 42.9,
	},
	{
		id: '13',
		title: 'The Wall',
		artist: 'Pink Floyd',
		imageUrl: 'https://picsum.photos/200/200?random=33',
		playCount: 5,
		duration: 42.9,
	},
	{
		id: '14',
		title: 'Innerspeaker',
		artist: 'Tame Impala',
		imageUrl: 'https://picsum.photos/200/200?random=34',
		playCount: 5,
		duration: 42.9,
	},
	{
		id: '15',
		title: 'Percepcion',
		artist: 'Hamada',
		imageUrl: 'https://picsum.photos/200/200?random=35',
		playCount: 5,
		duration: 42.9,
	},
];

export const mockTracks: Track[] = [
	{
		id: '1',
		title: 'Dragonborn',
		artist: 'The Elder Scrolls',
		album: 'Skyrim Soundtrack',
		imageUrl: 'https://picsum.photos/200/200?random=3',
		playCount: 234,
		duration: 4567,
	},
	{
		id: '2',
		title: 'We Will Rock You',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=7',
		playCount: 45,
		duration: 2.0,
	},
	{
		id: '3',
		title: "Don't Stop Me Now",
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=8',
		playCount: 40,
		duration: 3.3,
	},
	{
		id: '4',
		title: 'Radio Ga Ga',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=9',
		playCount: 35,
		duration: 5.5,
	},
	{
		id: '5',
		title: 'Somebody to Love',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=10',
		playCount: 30,
		duration: 4.6,
	},
	{
		id: '6',
		title: 'Crazy Little Thing Called Love',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=11',
		playCount: 25,
		duration: 2.4,
	},
	{
		id: '7',
		title: 'We Are the Champions',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=12',
		playCount: 20,
		duration: 3.0,
	},
	{
		id: '8',
		title: 'Killer Queen',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=13',
		playCount: 18,
		duration: 3.0,
	},
	{
		id: '9',
		title: 'Bicycle Race',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=14',
		playCount: 15,
		duration: 3.0,
	},
	{
		id: '10',
		title: 'Fat Bottomed Girls',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=15',
		playCount: 12,
		duration: 4.2,
	},
	{
		id: '11',
		title: 'Stairway to Heaven',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=16',
		playCount: 45,
		duration: 8.0,
	},
	{
		id: '12',
		title: 'Black Dog',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=17',
		playCount: 40,
		duration: 4.6,
	},
	{
		id: '13',
		title: 'Rock and Roll',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=18',
		playCount: 35,
		duration: 3.4,
	},
	{
		id: '14',
		title: 'Whole Lotta Love',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=19',
		playCount: 30,
		duration: 5.4,
	},
	{
		id: '15',
		title: 'Immigrant Song',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=20',
		playCount: 25,
		duration: 2.3,
	},
	{
		id: '16',
		title: 'Kashmir',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=21',
		playCount: 20,
		duration: 8.3,
	},
	{
		id: '17',
		title: 'Ramble On',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=22',
		playCount: 18,
		duration: 4.3,
	},
	{
		id: '18',
		title: 'Heartbreaker',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=23',
		playCount: 15,
		duration: 4.1,
	},
	{
		id: '19',
		title: 'Good Times Bad Times',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=24',
		playCount: 12,
		duration: 2.4,
	},
	{
		id: '20',
		title: 'Dazed and Confused',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=25',
		playCount: 10,
		duration: 6.2,
	},
];

const recentTracks: RecentTrack[] = [
	// Queen tracks
	{
		id: '1',
		title: 'Bohemian Rhapsody',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=1',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 30).toISOString(), // 30 minutes ago
		user: 'john_doe',
	},
	{
		id: '2',
		title: 'We Will Rock You',
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=7',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 45).toISOString(), // 45 minutes ago
		user: 'jane_smith',
	},
	{
		id: '3',
		title: "Don't Stop Me Now",
		artist: 'Queen',
		imageUrl: 'https://picsum.photos/200/200?random=8',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 90).toISOString(), // 1.5 hours ago
		user: 'bob_wilson',
	},

	// Led Zeppelin tracks
	{
		id: '4',
		title: 'Stairway to Heaven',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=16',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 60).toISOString(), // 1 hour ago
		user: 'jane_smith',
	},
	{
		id: '5',
		title: 'Black Dog',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=17',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 75).toISOString(), // 1.25 hours ago
		user: 'alice_brown',
	},
	{
		id: '6',
		title: 'Kashmir',
		artist: 'Led Zeppelin',
		imageUrl: 'https://picsum.photos/200/200?random=21',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 110).toISOString(), // 1.83 hours ago
		user: 'john_doe',
	},

	// The Elder Scrolls tracks
	{
		id: '7',
		title: 'Dragonborn',
		artist: 'The Elder Scrolls',
		imageUrl: 'https://picsum.photos/200/200?random=3',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 15).toISOString(), // 15 minutes ago
		user: 'charlie_davis',
	},
	{
		id: '8',
		title: 'Ancient Stones',
		artist: 'The Elder Scrolls',
		imageUrl: 'https://picsum.photos/200/200?random=31',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 50).toISOString(), // 50 minutes ago
		user: 'john_doe',
	},
	{
		id: '9',
		title: 'Secunda',
		artist: 'The Elder Scrolls',
		imageUrl: 'https://picsum.photos/200/200?random=32',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 120).toISOString(), // 2 hours ago
		user: 'jane_smith',
	},

	// Pink Floyd tracks
	{
		id: '10',
		title: 'Comfortably Numb',
		artist: 'Pink Floyd',
		imageUrl: 'https://picsum.photos/200/200?random=33',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 40).toISOString(), // 40 minutes ago
		user: 'bob_wilson',
	},
	{
		id: '11',
		title: 'Wish You Were Here',
		artist: 'Pink Floyd',
		imageUrl: 'https://picsum.photos/200/200?random=34',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 85).toISOString(), // 1.42 hours ago
		user: 'alice_brown',
	},

	// David Bowie tracks
	{
		id: '12',
		title: 'Space Oddity',
		artist: 'David Bowie',
		imageUrl: 'https://picsum.photos/200/200?random=35',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 55).toISOString(), // 55 minutes ago
		user: 'charlie_davis',
	},
	{
		id: '13',
		title: 'Heroes',
		artist: 'David Bowie',
		imageUrl: 'https://picsum.photos/200/200?random=36',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 130).toISOString(), // 2.17 hours ago
		user: 'john_doe',
	},

	// The Rolling Stones tracks
	{
		id: '14',
		title: 'Paint It Black',
		artist: 'The Rolling Stones',
		imageUrl: 'https://picsum.photos/200/200?random=37',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 70).toISOString(), // 1.17 hours ago
		user: 'jane_smith',
	},
	{
		id: '15',
		title: 'Sympathy for the Devil',
		artist: 'The Rolling Stones',
		imageUrl: 'https://picsum.photos/200/200?random=38',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 140).toISOString(), // 2.33 hours ago
		user: 'bob_wilson',
	},

	// Nirvana tracks
	{
		id: '16',
		title: 'Smells Like Teen Spirit',
		artist: 'Nirvana',
		imageUrl: 'https://picsum.photos/200/200?random=39',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 65).toISOString(), // 1.08 hours ago
		user: 'alice_brown',
	},
	{
		id: '17',
		title: 'Come As You Are',
		artist: 'Nirvana',
		imageUrl: 'https://picsum.photos/200/200?random=40',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 150).toISOString(), // 2.5 hours ago
		user: 'charlie_davis',
	},

	// Radiohead tracks
	{
		id: '18',
		title: 'Creep',
		artist: 'Radiohead',
		imageUrl: 'https://picsum.photos/200/200?random=41',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 80).toISOString(), // 1.33 hours ago
		user: 'john_doe',
	},
	{
		id: '19',
		title: 'Karma Police',
		artist: 'Radiohead',
		imageUrl: 'https://picsum.photos/200/200?random=42',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 160).toISOString(), // 2.67 hours ago
		user: 'jane_smith',
	},

	// Arctic Monkeys tracks
	{
		id: '20',
		title: 'Do I Wanna Know?',
		artist: 'Arctic Monkeys',
		imageUrl: 'https://picsum.photos/200/200?random=43',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 95).toISOString(), // 1.58 hours ago
		user: 'bob_wilson',
	},
	{
		id: '21',
		title: '505',
		artist: 'Arctic Monkeys',
		imageUrl: 'https://picsum.photos/200/200?random=44',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 170).toISOString(), // 2.83 hours ago
		user: 'alice_brown',
	},

	// Tame Impala tracks
	{
		id: '22',
		title: 'The Less I Know The Better',
		artist: 'Tame Impala',
		imageUrl: 'https://picsum.photos/200/200?random=45',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 105).toISOString(), // 1.75 hours ago
		user: 'charlie_davis',
	},
	{
		id: '23',
		title: 'Let It Happen',
		artist: 'Tame Impala',
		imageUrl: 'https://picsum.photos/200/200?random=46',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 180).toISOString(), // 3 hours ago
		user: 'john_doe',
	},

	// Other artists
	{
		id: '24',
		title: 'Hotel California',
		artist: 'Eagles',
		imageUrl: 'https://picsum.photos/200/200?random=47',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 125).toISOString(), // 2.08 hours ago
		user: 'jane_smith',
	},
	{
		id: '25',
		title: 'Sweet Caroline',
		artist: 'Neil Diamond',
		imageUrl: 'https://picsum.photos/200/200?random=48',
		lastPlayed: new Date(Date.now() - 1000 * 60 * 190).toISOString(), // 3.17 hours ago
		user: 'bob_wilson',
	},
];

// Generate 75 more recent tracks with data for artists in mockArtists
const generateMoreTracks = (): RecentTrack[] => {
	const tracks: RecentTrack[] = [];
	const users = [
		'john_doe',
		'jane_smith',
		'bob_wilson',
		'alice_brown',
		'charlie_davis',
	];

	// Get artist names from mockArtists
	const artistNames = mockArtists.map((artist) => artist.name);

	// Track titles for each artist
	const artistTracks: Record<string, string[]> = {
		'The Elder Scrolls': [
			'Far Horizons',
			'Streets of Whiterun',
			'Awake',
			'From Past to Present',
			'The City Gates',
			'Wind Guide You',
			'Skyrim Atmospheres',
			'Standing Stones',
			"Journey's End",
			'Frostfall',
			'Tundra',
			'Unbroken Road',
		],
		'Pink Floyd': [
			'Another Brick in the Wall',
			'Money',
			'Time',
			'Breathe',
			'The Great Gig in the Sky',
			'Us and Them',
			'Brain Damage',
			'Eclipse',
			'Hey You',
			'Shine On You Crazy Diamond',
			'Have a Cigar',
			'Welcome to the Machine',
		],
		'Led Zeppelin': [
			'Communication Breakdown',
			'Dazed and Confused',
			"Babe I'm Gonna Leave You",
			'Good Times Bad Times',
			'Ramble On',
			'Whole Lotta Love',
			'Immigrant Song',
			'Going to California',
			'When the Levee Breaks',
			'The Ocean',
			'Over the Hills and Far Away',
		],
		'David Bowie': [
			'Life on Mars?',
			'Starman',
			'Ziggy Stardust',
			'Rebel Rebel',
			"Let's Dance",
			'Modern Love',
			'Ashes to Ashes',
			'Changes',
			'The Man Who Sold the World',
			'Fame',
			'Young Americans',
			'China Girl',
		],
		Queen: [
			'Another One Bites the Dust',
			'Radio Ga Ga',
			'I Want to Break Free',
			'Somebody to Love',
			'Under Pressure',
			'Killer Queen',
			'Crazy Little Thing Called Love',
			"You're My Best Friend",
			'Fat Bottomed Girls',
			'Bicycle Race',
			'The Show Must Go On',
		],
		'The Rolling Stones': [
			'Gimme Shelter',
			'Start Me Up',
			'Brown Sugar',
			"Jumpin' Jack Flash",
			'Wild Horses',
			'Angie',
			'Ruby Tuesday',
			"(I Can't Get No) Satisfaction",
			'Honky Tonk Women',
			'Miss You',
			'Beast of Burden',
			'Emotional Rescue',
		],
		Nirvana: [
			'Lithium',
			'In Bloom',
			'Heart-Shaped Box',
			'All Apologies',
			'About a Girl',
			'Polly',
			'Something in the Way',
			'Drain You',
			'On a Plain',
			'Breed',
			'Lounge Act',
			'Territorial Pissings',
		],
		Radiohead: [
			'Paranoid Android',
			'No Surprises',
			'Fake Plastic Trees',
			'High and Dry',
			'Street Spirit (Fade Out)',
			'Idioteque',
			'Everything in Its Right Place',
			'Pyramid Song',
			'There There',
			'Weird Fishes/Arpeggi',
			'Lotus Flower',
			'15 Step',
		],
		'Arctic Monkeys': [
			'R U Mine?',
			"Why'd You Only Call Me When You're High?",
			'Arabella',
			'Fluorescent Adolescent',
			'I Bet You Look Good on the Dancefloor',
			'When the Sun Goes Down',
			'Brianstorm',
			'Crying Lightning',
			'Cornerstone',
			'Snap Out of It',
			'Four Out of Five',
			'Tranquility Base Hotel & Casino',
		],
		'Tame Impala': [
			'Feels Like We Only Go Backwards',
			'Elephant',
			'Borderline',
			'Lost in Yesterday',
			'New Person, Same Old Mistakes',
			'Eventually',
			"Yes I'm Changing",
			'Posthumous Forgiveness',
			'Breathe Deeper',
			'Is It True',
			'One More Year',
			'It Might Be Time',
		],
	};

	// Starting ID (after the hardcoded tracks)
	let nextId = 26;

	// Generate tracks for each artist
	for (const artist of artistNames) {
		// Get track titles for this artist
		const artistTrackTitles = artistTracks[artist] || [];

		// Generate 7-8 tracks for each artist (75-80 tracks total for 10 artists)
		const tracksToGenerate = 7 + Math.floor(Math.random() * 2); // 7 or 8 tracks

		for (let i = 0; i < tracksToGenerate; i++) {
			// Get a random track title for this artist, or use a generic one if none are defined
			let trackTitle = 'Unknown Track';
			if (artistTrackTitles.length > 0) {
				trackTitle =
					artistTrackTitles[
						Math.floor(Math.random() * artistTrackTitles.length)
					];
			}

			const randomUser = users[Math.floor(Math.random() * users.length)];
			// Random time between 3 hours and 7 days ago
			const hoursAgo = 3 + Math.floor(Math.random() * (24 * 7 - 3));

			tracks.push({
				id: nextId.toString(),
				title: trackTitle,
				artist: artist,
				imageUrl: `https://picsum.photos/200/200?random=${nextId + 50}`, // Offset to avoid duplicates
				lastPlayed: new Date(
					Date.now() - 1000 * 60 * 60 * hoursAgo
				).toISOString(),
				user: randomUser,
			});

			nextId++;
		}
	}

	return tracks;
};

export const fetchMusicStats = async (
	timeRange: TimeRange,
	pulseRange: PulseTimeRange = '12months',
	username: string | null = null
): Promise<MusicStats> => {
	const allRecentTracks = [...recentTracks, ...generateMoreTracks()].filter(
		(track) => {
			// Filter by time range
			let passesTimeFilter = true;
			if (timeRange === 'today') {
				const today = new Date().toDateString();
				passesTimeFilter =
					new Date(track.lastPlayed).toDateString() === today;
			} else if (timeRange === 'week') {
				const weekAgo = new Date();
				weekAgo.setDate(weekAgo.getDate() - 7);
				passesTimeFilter = new Date(track.lastPlayed) >= weekAgo;
			} else if (timeRange === 'month') {
				const monthAgo = new Date();
				monthAgo.setMonth(monthAgo.getMonth() - 1);
				passesTimeFilter = new Date(track.lastPlayed) >= monthAgo;
			} else if (timeRange === 'year') {
				const yearAgo = new Date();
				yearAgo.setFullYear(yearAgo.getFullYear() - 1);
				passesTimeFilter = new Date(track.lastPlayed) >= yearAgo;
			}

			// Filter by username if provided
			const passesUserFilter = username ? track.user === username : true;

			return passesTimeFilter && passesUserFilter;
		}
	);

	// Sort tracks by lastPlayed in descending order (newest first)
	const sortedRecentTracks = allRecentTracks.sort(
		(a, b) =>
			new Date(b.lastPlayed).getTime() - new Date(a.lastPlayed).getTime()
	);

	// Generate pulse data based on the selected range
	const generatePulseData = () => {
		const now = new Date();
		const data: PulseData[] = [];
		let count = 12;

		switch (pulseRange) {
			case '12days':
				for (let i = 0; i < count; i++) {
					const date = new Date(now);
					date.setDate(date.getDate() - i);
					data.unshift({
						period: date.toLocaleString('default', {
							month: 'short',
							day: 'numeric',
						}),
						playCount: Math.floor(Math.random() * 1000) + 100,
					});
				}
				break;
			case '12weeks':
				for (let i = 0; i < count; i++) {
					const date = new Date(now);
					date.setDate(date.getDate() - i * 7);
					data.unshift({
						period: `Week ${count - i}`,
						playCount: Math.floor(Math.random() * 1000) + 100,
					});
				}
				break;
			case '12months':
				for (let i = 0; i < count; i++) {
					const date = new Date(now);
					date.setMonth(date.getMonth() - i);
					data.unshift({
						period: date.toLocaleString('default', {
							month: 'short',
							year: 'numeric',
						}),
						playCount: Math.floor(Math.random() * 1000) + 100,
					});
				}
				break;
			case '12years':
				for (let i = 0; i < count; i++) {
					const date = new Date(now);
					date.setFullYear(date.getFullYear() - i);
					data.unshift({
						period: date.getFullYear().toString(),
						playCount: Math.floor(Math.random() * 1000) + 100,
					});
				}
				break;
		}

		return data;
	};

	return {
		topArtists: mockArtists,
		topTracks: mockTracks,
		topAlbums: mockAlbums,
		recentTracks: sortedRecentTracks,
		timeStats: {
			today: { playCount: 7, duration: 45 },
			week: { playCount: 126, duration: 840 },
			month: { playCount: 199, duration: 1320 },
			year: { playCount: 199, duration: 1320 },
			all: { playCount: 199, duration: 1320 },
		},
		pulseData: generatePulseData(),
	};
};

export const fetchArtistDetails = async (
	artistId: string,
	username: string | null = null
): Promise<ArtistDetails> => {
	console.log('fetchArtistDetails: ', artistId, ' username: ', username);
	const artist = mockArtists.find((a) => a.id === artistId);
	if (!artist) {
		throw new Error('Artist not found');
	}

	// Get top tracks for this artist
	const artistTracks = mockTracks
		.filter((t) => t.artist === artist.name)
		.sort((a, b) => b.playCount - a.playCount)
		.slice(0, 10);

	// Get albums for this artist
	const artistAlbums = mockAlbums
		.filter((a) => a.artist === artist.name)
		.sort((a, b) => b.playCount - a.playCount);

	// Get recent listens for this artist
	const allRecentTracks = [...recentTracks, ...generateMoreTracks()];
	const artistRecentListens = allRecentTracks
		.filter(
			(t) =>
				t.artist === artist.name &&
				(username ? t.user === username : true)
		)
		.sort(
			(a, b) =>
				new Date(b.lastPlayed).getTime() -
				new Date(a.lastPlayed).getTime()
		)
		.slice(0, 20); // Limit to 20 recent listens

	// Artist descriptions
	const artistDescriptions: Record<string, string> = {
		'1': 'The Elder Scrolls is a series of action role-playing video games developed by Bethesda Game Studios. The soundtrack, composed by Jeremy Soule, is known for its atmospheric and immersive orchestral pieces that enhance the fantasy world of the games.',
		'2': 'Pink Floyd were an English rock band formed in London in 1965. Gaining an early following as one of the first British psychedelic groups, they were distinguished by their extended compositions, sonic experimentation, philosophical lyrics and elaborate live shows.',
		'3': 'Led Zeppelin were an English rock band formed in London in 1968. With their heavy, guitar-driven sound, they are regularly cited as one of the progenitors of heavy metal, although their style drew from a variety of influences, including blues and folk music.',
		'4': 'David Bowie was an English singer-songwriter and actor. He was a leading figure in the music industry and is regarded as one of the most influential musicians of the 20th century. Known for his innovative work during the 1970s, Bowie is famous for his distinctive voice and the intellectual depth of his work.',
		'5': 'Queen are a British rock band formed in London in 1970. Their classic line-up was Freddie Mercury, Brian May, Roger Taylor and John Deacon. Their earliest works were influenced by progressive rock, hard rock and heavy metal, but the band gradually ventured into more conventional and radio-friendly works.',
		'6': 'The Rolling Stones are an English rock band formed in London in 1962. Diverging from the pop rock of the early 1960s, the Rolling Stones pioneered the gritty, heavier-driven sound that came to define hard rock. Their first stable line-up was vocalist Mick Jagger, multi-instrumentalist Brian Jones, guitarist Keith Richards, drummer Charlie Watts, and bassist Bill Wyman.',
		'7': "Nirvana was an American rock band formed in Aberdeen, Washington in 1987. Founded by lead singer and guitarist Kurt Cobain and bassist Krist Novoselic, the band went through a succession of drummers, most notably Dave Grohl, who joined in 1990. Nirvana's success popularized alternative rock, and they were often referenced as the figurehead band of Generation X.",
		'8': "Radiohead are an English rock band formed in Abingdon, Oxfordshire, in 1985. The band consists of Thom Yorke, brothers Jonny Greenwood and Colin Greenwood, Ed O'Brien and Philip Selway. They have worked with producer Nigel Godrich and cover artist Stanley Donwood since 1994.",
		'9': "Arctic Monkeys are an English rock band formed in Sheffield in 2002. The group consists of Alex Turner, Jamie Cook, Nick O'Malley, and Matt Helders. Former band member Andy Nicholson left the band in 2006 shortly after their debut album was released.",
		'10': "Tame Impala is the psychedelic music project of Australian multi-instrumentalist Kevin Parker. In the recording studio, Parker writes, records, performs, and produces all of the project's music. As a touring act, Tame Impala consists of Parker, Dominic Simper, Jay Watson, Cam Avery, and Julien Barbagallo.",
	};

	return {
		...artist,
		topTracks: artistTracks,
		albums: artistAlbums,
		recentListens: artistRecentListens,
		description:
			artistDescriptions[artistId] || 'No description available.',
	};
};

const generateApiKey = (): string => {
	const chars =
		'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	let key = '';
	for (let i = 0; i < 32; i++) {
		key += chars.charAt(Math.floor(Math.random() * chars.length));
	}
	return key;
};

export const createApiKey = async (
	userId: string,
	label: string
): Promise<ApiKey> => {
	// Simulate API delay
	await new Promise((resolve) => setTimeout(resolve, 500));

	const newApiKey: ApiKey = {
		id: Math.random().toString(36).substring(7),
		label: `${label}-${userId}`,
		key: generateApiKey(),
		createdAt: new Date().toISOString(),
	};

	return newApiKey;
};

export const getUserProfile = async (userId: string): Promise<User> => {
	// Simulate API delay
	await new Promise((resolve) => setTimeout(resolve, 500));

	return {
		id: userId,
		name: 'John Doe',
		imageUrl: 'https://picsum.photos/32/32?random=1',
		lastActive: new Date().toISOString(),
		apiKeys: [
			{
				id: '1',
				label: 'Personal App',
				key: generateApiKey(),
				createdAt: new Date(
					Date.now() - 1000 * 60 * 60 * 24 * 7
				).toISOString(), // 7 days ago
				lastUsed: new Date().toISOString(),
			},
			{
				id: '2',
				label: 'Work Project',
				key: generateApiKey(),
				createdAt: new Date(
					Date.now() - 1000 * 60 * 60 * 24 * 30
				).toISOString(), // 30 days ago
				lastUsed: new Date(
					Date.now() - 1000 * 60 * 60 * 24 * 2
				).toISOString(), // 2 days ago
			},
		],
		stats: {
			totalPlays: 1234,
			totalDuration: 45678,
			topArtists: mockArtists.slice(0, 5),
			topAlbums: mockAlbums.slice(0, 5),
			topTracks: mockTracks.slice(0, 5),
		},
	};
};

export const fetchAlbumDetails = async (
	albumId: string,
	username: string | null = null
): Promise<AlbumDetails> => {
	console.log('fetchAlbumDetails: ', albumId, ' username: ', username);
	const album = mockAlbums.find((a) => a.id === albumId);
	if (!album) {
		throw new Error('Album not found');
	}

	// Get top tracks for this album
	const albumTracks = mockTracks
		.filter((t) => t.artist === album.artist)
		.sort((a, b) => b.playCount - a.playCount)
		.slice(0, 10);

	return {
		...album,
		topTracks: albumTracks,
	};
};

// Simulate import process with a delay
export const importUserData = async (
	file: File,
	format: string
): Promise<boolean> => {
	// Simulate processing time (2-4 seconds)
	const processingTime = 2000 + Math.random() * 2000;
	await new Promise((resolve) => setTimeout(resolve, processingTime));

	// Use the file parameter to suppress unused variable warning
	console.log(`Importing file: ${file.name} in ${format} format`);

	// Always return success in mock implementation
	return true;
};
