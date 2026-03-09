import { invoke } from "@tauri-apps/api/core";
import { load } from "../features/load.ts";
import { switchView } from "../features/switchView.ts";
import type { Callback } from "../types.ts";
import { element } from "./element.ts";

export function createLiProject(name: string): HTMLLIElement {
	const li = document.createElement("li");

	const mainBtn = createButton(name, async () => {
		await load("project", { name });
		element.projectName.textContent = `Project : ${name}`;
		switchView("listProject", "editorTask");
	});

	const deleteBtn = createButton("X", async () => {
		await invoke("delete_project", { name });
		li.remove();
	});

	li.append(mainBtn, deleteBtn);
	return li;
}

export function createButton(
	text: string,
	eventListener: Callback,
): HTMLButtonElement {
	const button = document.createElement("button");
	button.textContent = text;
	button.type = "button";
	button.addEventListener("click", eventListener);
	return button;
}
