import { element } from "../class/element.ts";


const viewsEditor = {
	editorTask: ["menuEditorEditor", "editorTask", "addTask", "inputAddTask"],
	listProject: ["menuEditorList", "listProject"],
	createProject: ["menuEditorCreate", "createProject"],
} as const;

function toggleVisibility(
	keys: (typeof viewsEditor)[Keys],
	hide: boolean,
): void {
	for (const key of keys) {
		element[key].classList.toggle("hide", hide);
	}
}

function switchView(oldViews: Keys, newView: Keys): void {
	toggleVisibility(viewsEditor[oldViews], true);
	toggleVisibility(viewsEditor[newView], false);
}

export type Keys = keyof typeof viewsEditor;

export { switchView };
