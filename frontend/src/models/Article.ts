export interface Article {
    title: string;
    date: number; // saving as unix time makes it a bit easier to work with
    content: string;
}