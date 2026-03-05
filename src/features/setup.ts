import { invoke } from "@tauri-apps/api/core";
import { createLiProject } from "@/class/create.ts";
import { element } from "@/class/element.ts";
import { markdown } from "@/features/markdown.ts";

async function setup(): Promise<void> {
	//---Load Project List
	const { listProject, editorTask, helpPanel } = element;
	const projects = await invoke<string[]>("get_list_projects");

	for (const name of projects) {
		const actualLi = createLiProject(name);
		listProject.append(actualLi);
	}

	//---Load Event in editor

  editorTask.addEventListener("input", () => {
    markdown();
    invoke("save_text_task", { text: editorTask.innerHTML }).catch((e) => {
			throw new Error(e);
		});
	});
	document.addEventListener("selectionchange", (e) => {
		if (e.target === editorTask) {
			//markdownReverse();
		}
	});

	//---Load Draft
	helpPanel.innerHTML = await invoke("get_draft");
}

await setup();
