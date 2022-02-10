# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0257`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "a"
				loc: SourceLocation esprima/invalid-syntax/migrated_0257/input.js 1:14-1:15 (a)
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0257/input.js 1:14-1:15
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "package"
				loc: SourceLocation esprima/invalid-syntax/migrated_0257/input.js 1:16-1:23 (package)
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0257/input.js 1:16-1:23
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
				message: RAW_MARKUP {value: "Expected a semicolon or a line terminator"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0257/input.js>
				end: Position 1:15
				start: Position 1:16
			}
		}
	]
	directives: [
		JSDirective {
			value: "use strict"
			loc: SourceLocation esprima/invalid-syntax/migrated_0257/input.js 1:0-1:13
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0257/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0257/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0257/input.js:1:16 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    'use strict'; a package
                    ^


```