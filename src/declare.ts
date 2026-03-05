/** biome-ignore-all lint/nursery/useGlobalThis: use only in browser */
import { formCreateProject, inputEvent, saveDraft } from "./features/event.ts";
import { type Keys, switchView } from "./features/switchView.ts";

declare global {
	interface Window {
		switchView: (oldViews: Keys, newView: Keys) => void;
		formCreateProject: (event: Event) => void;
		inputEvent: () => Promise<void>;
		saveDraft: (textDraft: string) => void;
	}
}

window.switchView = switchView;
window.inputEvent = inputEvent;
window.formCreateProject = formCreateProject;
window.saveDraft = saveDraft;
