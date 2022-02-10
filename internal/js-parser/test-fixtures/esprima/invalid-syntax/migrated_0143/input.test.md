# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0143`

### `ast`

```javascript
JSRoot {
	body: [
		JSSwitchStatement {
			cases: [
				JSSwitchCase {
					consequent: []
					loc: SourceLocation esprima/invalid-syntax/migrated_0143/input.js 1:13-1:21
				}
				JSSwitchCase {
					consequent: []
					loc: SourceLocation esprima/invalid-syntax/migrated_0143/input.js 1:22-1:30
				}
			]
			discriminant: JSReferenceIdentifier {
				name: "c"
				loc: SourceLocation esprima/invalid-syntax/migrated_0143/input.js 1:8-1:9 (c)
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0143/input.js 1:0-1:32
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
				message: RAW_MARKUP {value: "Multiple default clauses"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0143/input.js>
				end: Position 1:22
				start: Position 1:22
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0143/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0143/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0143/input.js:1:22 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Multiple default clauses

    switch (c) { default: default: }
                          ^


```