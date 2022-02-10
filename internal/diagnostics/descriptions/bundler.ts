import {createDiagnosticsCategory} from "./index";
import {StaticMarkup, markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export const bundler = createDiagnosticsCategory({
	DETECTED_CYCLE: (
		localName: string,
		target: string,
		culprit: string,
		path: string[],
	) => {
		function formatPart(part: string, index?: number): StaticMarkup {
			const tagged = markup`<filelink target="${part}" />`;
			if (part === culprit) {
				return markup`<highlight i="0" legend>${tagged}</highlight>`;
			} else if (part === target) {
				return markup`<highlight i="1" legend>${tagged}</highlight>`;
			} else if (index === 0) {
				return markup`${tagged} <inverse> ENTRY </inverse>`;
			} else {
				return tagged;
			}
		}

		return {
			category: DIAGNOSTIC_CATEGORIES["bundler/moduleCycle"],
			message: markup`The variable <emphasis>${localName}</emphasis> won't be initialized yet`,
			advice: [
				{
					type: "log",
					category: "info",
					text: markup`This is because the module it belongs to wont be executed yet. This is due to a circular dependency creating a module cycle.`,
				},
				{
					type: "log",
					category: "info",
					text: markup`The likely cause is the file ${formatPart(culprit)} that was required by ${formatPart(
						target,
					)}:`,
				},
				{
					type: "list",
					reverse: true,
					ordered: true,
					truncate: false,
					list: path.map(formatPart),
				},
			],
		};
	},
});