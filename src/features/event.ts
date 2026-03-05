import { invoke } from "@tauri-apps/api/core";
import type { Task } from "@/types.ts";
import { createButtonListTask, createLiProject } from "../class/create.ts";
import { element } from "../class/element.ts";
import { switchView } from "./switchView.ts";

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
  console.log("a")

	if (input.value.length === 0) {
		throw new Error("Error name less 1 char");
	}
	const name = input.value;
	const id = await invoke<number>("create_task", { name });

  const { listTask } = element;
	listTask.append(createButtonListTask(name, id));
}

async function saveDraft(text: string): Promise<void> {
	await invoke("save_draft", { text });
}

export { formCreateProject, inputEvent, saveDraft };
