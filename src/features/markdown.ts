import type { MarkdownData } from "@/types.ts";
import { value } from "../class/valueGlobal.ts";

export const markStyle: Record<string, string> = {
	"*": "strong",
	_: "i",
};

export function markdown(): void {
	const range = document.getSelection()?.getRangeAt(0);
	if (!range) return;

	const { startContainer: node, startOffset: offset } = range;
	const text = node.textContent;
	if (!text || text.length < 2) return;

	const ch = text.charAt(offset - 1);
	const tag = markStyle[ch];
	if (!tag || text.charAt(offset - 2) !== ch) return;

	if (!value.markdownData.has(ch)) {
		value.markdownData.set(ch, { pos: offset, node });
		return;
	}

	const { pos, node: prev } = value.markdownData.get(ch) as MarkdownData;
	if (!(offset - 2 > pos || pos > offset + 2)) return;

	const pt = prev.textContent;
	if (!pt || pt[pos - 1] !== ch || pt[pos - 2] !== ch) {
		value.markdownData.set(ch, { pos: offset, node });
		return;
	}

	const r = document.createRange();
	pos < offset
		? (r.setStart(prev, pos - 2), r.setEnd(node, offset))
		: (r.setStart(node, offset), r.setEnd(prev, pos));

  const el = document.createElement(tag);
  el.appendChild(r.extractContents());
  
	r.insertNode(el);

  const first = el.firstChild as Text;
  first.deleteData(first.data.length - 2, 2)
  first.deleteData(0,2)


	const after = document.createTextNode("\u200B");
	el.before(document.createTextNode("\u200B"));
	el.after(after);

	range.setStart(after, 1);
	value.markdownData.delete(ch);
}
