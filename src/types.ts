export type Callback = (data?: any) => void | Promise<void>;

export interface Task {
    id: number;
    name: string;
    text: string;
}
export interface ActualMarkdownData {
    char: string;
    pos: number;
    node: Node;
    range: Range;
}

export interface MarkdownData {
    pos: number;
    node: Node;
}
