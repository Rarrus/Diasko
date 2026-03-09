import { invoke } from "@tauri-apps/api/core";
import { createButton, createLiProject } from "../class/create.ts";
import { element } from "../class/element.ts";
import { switchView } from "./switchView.ts";
import { load } from "./load.ts";

async function formCreateProject(event: Event): Promise<void> {
	event?.preventDefault();

	const form = element.createProject;
	const data = new FormData(form);
	const name = data.get("project_name")?.toString();

	if (!name) {
		throw new Error("Project name is required");
	}
	await invoke("create_project", { name });
	form.reset();
	switchView("createProject", "listProject");
	const list = element.listProject;
	const actualLi = createLiProject(name);
	list.append(actualLi);
}

async function inputEvent(): Promise<void> {
	const input = element.inputAddTask;

	const name = input.value;
	if (name.length === 0) {
		throw new Error("Error name less 1 char");
	}
	const id = await invoke<number>("create_task", { name });

	element.listTask.append(createButton(name, () => load("task", { id })));
}

async function saveDraft(text: string): Promise<void> {
	await invoke("save_draft", { text });
}



export { formCreateProject, inputEvent, saveDraft };
