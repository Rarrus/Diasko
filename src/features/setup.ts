import { invoke } from "@tauri-apps/api/core";
import { createLiProject } from "../class/create.ts";
import { markdown } from "./markdown.ts";

const get = <T extends HTMLElement>(id: string) => document.getElementById(id) as T ?? (() => { throw new Error(`element dont exist ${id}`) })
class diasko {
	addons = get("addons");
	pomodoro = get("pomodoro");
	music = get("music");
	listTask = get<HTMLUListElement>("list_task");
	inputAddTask = get<HTMLInputElement>("input_add_task");
	treeTask = get("tree_task");
	menu = get("menu");
	menuEditor = get("menu_editor");
	editorPanel = get("editor_panel");
	helpMenu = get("help_menu");
	helpPanel = get("help_panel");
	switchViewCreate = get("switch_view_create");
	switchViewList = get("switch_view_list");
	listProject = get<HTMLUListElement>("list_project");
	createProject = get<HTMLFormElement>("create_project");
	editorTask = get<HTMLDivElement>("editor_task");
	switchViewListEditor = get<HTMLButtonElement>("switch_view_list_editor");
	menuEditorEditor = get<HTMLDivElement>("menu_editor_editor");
	menuEditorCreate = get<HTMLDivElement>("menu_editor_create");
	menuEditorList = get<HTMLDivElement>("menu_editor_list");
	nameTask = get("name_task");
	projectName = get("project_name");
	addTask = get("add_task");
	taskPanel = get("task")

	constructor() {
		await this.setup()
	}

	async setup(): Promise<void> {
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
}
}


new diasko();
