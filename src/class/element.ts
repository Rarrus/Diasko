function g<T extends HTMLElement = HTMLElement>(id: string): T {
    const result = document.getElementById(id);
    if (!result) {
        throw new Error(`Element id ${id} not found`);
    }
    return result as T;
}

export const element = {
    addons: g("addons"),
    pomodoro: g("pomodoro"),
    music: g("music"),
    listTask: g<HTMLUListElement>("list_task"),
    inputAddTask: g<HTMLInputElement>("input_add_task"),
    treeTask: g("tree_task"),
    menu: g("menu"),
    menuEditor: g("menu_editor"),
    editorPanel: g("editor_panel"),
    helpMenu: g("help_menu"),
    helpPanel: g("help_panel"),
    switchViewCreate: g("switch_view_create"),
    switchViewList: g("switch_view_list"),
    listProject: g<HTMLUListElement>("list_project"),
    createProject: g<HTMLFormElement>("create_project"),
    editorTask: g<HTMLDivElement>("editor_task"),
    switchViewListEditor: g<HTMLButtonElement>("switch_view_list_editor"),
    menuEditorEditor: g<HTMLDivElement>("menu_editor_editor"),
    menuEditorCreate: g<HTMLDivElement>("menu_editor_create"),
    menuEditorList: g<HTMLDivElement>("menu_editor_list"),
    nameTask: g("name_task"),
    projectName: g("project_name"),
    addTask: g("add_task"),
} as const;
