export const formatDuration = (seconds: number): string => {
	const minutes = Math.floor(seconds / 60000);

	if (minutes < 60) {
		return `${minutes}m`;
	} else {
		const hours = Math.floor(minutes / 60);
		const remainingMinutes = minutes % 60;
		return remainingMinutes > 0
			? `${hours}h ${remainingMinutes}m`
			: `${hours}h`;
	}
};

export const formatMillisecondsToMinutes = (ms: number): string => {
	const minutes = Math.floor(ms / 60000);
	const seconds = Math.floor((ms % 60000) / 1000);
	return `${minutes}:${seconds.toString().padStart(2, '0')}`;
};

export const formatDates = (date: string): string => {
	const dateObj = new Date(date);
	return dateObj.toLocaleDateString('fr-FR', {
		month: 'numeric',
		day: 'numeric',
		year: 'numeric',
	});
};
export const formatTimeAgo = (timestamp: string): string => {
	const date = new Date(timestamp);
	const now = new Date();
	const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

	if (diffInSeconds < 60) {
		return 'just now';
	} else if (diffInSeconds < 3600) {
		const minutes = Math.floor(diffInSeconds / 60);
		return `${minutes}m ago`;
	} else if (diffInSeconds < 86400) {
		const hours = Math.floor(diffInSeconds / 3600);
		return `${hours}h ago`;
	} else if (diffInSeconds < 604800) {
		const days = Math.floor(diffInSeconds / 86400);
		return `${days}d ago`;
	} else if (diffInSeconds < 2592000) {
		const weeks = Math.floor(diffInSeconds / 604800);
		return `${weeks}w ago`;
	} else if (diffInSeconds < 31536000) {
		const months = Math.floor(diffInSeconds / 2592000);
		return `${months}mo ago`;
	} else {
		const years = Math.floor(diffInSeconds / 31536000);
		return `${years}y ago`;
	}
};

const formatTimeOfDay = (date: string): string => {
	const dateObj = new Date(date);
	return dateObj.toLocaleTimeString('fr-FR', {
		hour: '2-digit',
		minute: '2-digit',
	});
};

export const getWeekNumber = (period: string): string => {
	const date = new Date(period);
	const firstDayOfYear = new Date(date.getFullYear(), 0, 1);
	const pastDaysOfYear =
		(date.getTime() - firstDayOfYear.getTime()) / (1000 * 60 * 60 * 24);
	return Math.ceil(
		(pastDaysOfYear + firstDayOfYear.getDay() + 1) / 7
	).toString();
};

export const formatDateBasedOnPeriod = (
	period: string,
	date: string
): string => {
	switch (period) {
		case 'today':
			return formatTimeOfDay(date);
		case 'week':
			return date;
		case 'month':
			return `Week ${getWeekNumber(date)}`;
		case 'year':
			return date;
		default:
			return date;
	}
};
