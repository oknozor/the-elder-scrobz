export type TimeRange = "today" | "week" | "month" | "year" | "all";

export interface PaginatedResponse<T> {
    data: T[];
    total: number;
    page: number;
    page_size: number;
}
