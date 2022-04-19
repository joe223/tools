import { IndentStyle, PlaygroundState } from "./types";
import { Dispatch, SetStateAction } from "react";

interface Props {
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>,
	indentStyle: IndentStyle,
	indentWidth: number,
}

export default function IndentStyleSelect(
	{ indentStyle, indentWidth, setPlaygroundState }: Props,
) {
	return (
		<div className="pl-5 pb-5 sm:p-5 flex">
			<fieldset className="space-y-5">
				<div>
					<label
						htmlFor="indentStyle"
						className="block text-sm font-medium text-gray-700"
					>
						Indent Style
					</label>
					<select
						id="location"
						name="location"
						className="w-[100px] mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
						value={indentStyle}
						onChange={(e) => {
							setPlaygroundState((state) => ({
								...state,
								indentStyle: e.target.value as IndentStyle,
							}));
						}}
					>
						<option value={IndentStyle.Tab}>Tabs</option>
						<option value={IndentStyle.Space}>Spaces</option>
					</select>
				</div>
			</fieldset>
			{indentStyle === IndentStyle.Space && (
				<div className="pl-4">
					<label
						htmlFor="indentWidth"
						className="block text-sm font-medium text-gray-700"
					>
						Indent Width
					</label>
					<input
						type="number"
						name="indentWidth"
						id="indentWidth"
						className="w-[65px] mt-1 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
						value={indentWidth}
						onChange={(e) => {
							setPlaygroundState((state) => ({
								...state,
								indentWidth: parseInt(e.target.value),
							}));
						}}
					/>
				</div>
			)}
		</div>
	);
}
