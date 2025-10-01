export type TimeRange = "today" | "week" | "month" | "year" | "all";

export interface PaginatedResponse<T> {
    data: T[];
    total: number;
    page: number;
    page_size: number;
    total_pages: number;
    previous_page: number | null;
    next_page: number | null;
    last_page: number;
}
