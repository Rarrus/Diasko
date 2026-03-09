import { element } from "../class/element.ts";


const viewsEditor = {
	editorTask: [element.menuEditorEditor, element.editorTask, element.taskPanel],
	listProject: [element.menuEditorList, element.listProject],
	createProject: [element.menuEditorCreate, element.createProject],
} as const;

function switchView(oldViews: Keys, newView: Keys): void {
	const keys = [...viewsEditor[oldViews], ...viewsEditor[newView]]
	for (const key of keys) {
		key.classList.toggle("hide");
	}
}

export type Keys = keyof typeof viewsEditor;

export { switchView };
