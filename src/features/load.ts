import { invoke } from "@tauri-apps/api/core";
import {createButtonListTask } from "../class/create.ts";
import { element } from "../class/element.ts";
import type { Task } from "../types.ts";

type Action = "task" | "project";
type Args<K extends Action> = K extends "task"
	? { id: number }
	: { name: string };

export async function load<K extends Action>(
	action: K,
	args: Args<K>,
): Promise<void> {
	const [task, listChildren] = await invoke<[Task, Task[]]>(
		`go_to_${action}`,
		args,
	);
	const { name, text } = task;
	const { nameTask, listTask, editorTask } = element;

	nameTask.textContent = `Task : ${name}`;
  listTask.innerText = "";
  editorTask.innerHTML = text;

  for (const child of listChildren) {
    const nameChild = child.name
		const { id } = child;
		listTask.append(createButtonListTask(nameChild, id));
	}
}
