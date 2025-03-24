import type { User } from '@/types/music'
import { mockArtists, mockAlbums, mockTracks } from './mockData'

export const mockUsers: User[] = [
  {
    id: '1',
    name: 'John Doe',
    imageUrl: 'https://picsum.photos/32/32?random=1',
    lastActive: new Date().toISOString(),
    apiKeys: [],
    stats: {
      totalPlays: 1234,
      totalDuration: 45678,
      topArtists: mockArtists.slice(0, 5),
      topAlbums: mockAlbums.slice(0, 5),
      topTracks: mockTracks.slice(0, 5)
    }
  },
  {
    id: '2',
    name: 'Jane Smith',
    imageUrl: 'https://picsum.photos/32/32?random=2',
    lastActive: new Date().toISOString(),
    apiKeys: [],
    stats: {
      totalPlays: 987,
      totalDuration: 34567,
      topArtists: mockArtists.slice(5, 10),
      topAlbums: mockAlbums.slice(5, 10),
      topTracks: mockTracks.slice(5, 10)
    }
  },
  {
    id: '3',
    name: 'Bob Johnson',
    imageUrl: 'https://picsum.photos/32/32?random=3',
    lastActive: new Date().toISOString(),
    apiKeys: [],
    stats: {
      totalPlays: 567,
      totalDuration: 23456,
      topArtists: mockArtists.slice(10, 15),
      topAlbums: mockAlbums.slice(10, 15),
      topTracks: mockTracks.slice(10, 15)
    }
  },
  {
    id: '4',
    name: 'Alice Brown',
    imageUrl: 'https://picsum.photos/32/32?random=4',
    lastActive: new Date().toISOString(),
    apiKeys: [],
    stats: {
      totalPlays: 432,
      totalDuration: 12345,
      topArtists: mockArtists.slice(15, 20),
      topAlbums: mockAlbums.slice(15, 20),
      topTracks: mockTracks.slice(15, 20)
    }
  },
  {
    id: '5',
    name: 'Charlie Wilson',
    imageUrl: 'https://picsum.photos/32/32?random=5',
    lastActive: new Date().toISOString(),
    apiKeys: [],
    stats: {
      totalPlays: 321,
      totalDuration: 9876,
      topArtists: mockArtists.slice(20, 25),
      topAlbums: mockAlbums.slice(20, 25),
      topTracks: mockTracks.slice(20, 25)
    }
  }
]

export const fetchUsers = async (): Promise<User[]> => {
  return mockUsers
} 