import type { MarkdownData } from "@/types";

interface ValueGlobal {
	markdownData: Map<string, MarkdownData>;
}

export const value: ValueGlobal = {
	markdownData: new Map<string, MarkdownData>(),
};
