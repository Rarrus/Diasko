import { invoke } from "@tauri-apps/api/core";
import { createButton } from "../class/create.ts";
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
    const [task, list] = await invoke<[Task, Task[]]>(
        `go_to_${action}`,
        args,
    );
    const tag = [element.nameTask, element.editorTask, element.listTask];
    const stringText = [task.name, task.text, ""];

    tag.forEach((value, index) => value.textContent = stringText[index])
    list.forEach((value) => createButton(value.name, () => load("task", { id: value.id })));
}
