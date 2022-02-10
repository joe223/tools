# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 457`

### `ast`

```javascript
JSRoot {
	body: [
		JSSwitchStatement {
			cases: [
				JSSwitchCase {
					consequent: [
						JSContinueStatement {
							loc: SourceLocation core/uncategorised/457/input.js 1:22-1:31
						}
					]
					loc: SourceLocation core/uncategorised/457/input.js 1:13-1:31
				}
			]
			discriminant: JSReferenceIdentifier {
				name: "x"
				loc: SourceLocation core/uncategorised/457/input.js 1:8-1:9 (x)
			}
			loc: SourceLocation core/uncategorised/457/input.js 1:0-1:33
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "No loop label found"}
			}
			location: {
				language: "js"
				path: UIDPath<core/uncategorised/457/input.js>
				end: Position 1:22
				start: Position 1:22
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/457/input.js>
	loc: SourceLocation core/uncategorised/457/input.js 1:0-1:33
}
```

### `diagnostics`

```

 core/uncategorised/457/input.js:1:22 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ No loop label found

    switch (x) { default: continue; }
                          ^


```