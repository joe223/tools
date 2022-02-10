# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0239`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "static"
				loc: SourceLocation esprima/invalid-syntax/migrated_0239/input.js 1:23-1:29 (static)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation esprima/invalid-syntax/migrated_0239/input.js 1:32-1:35
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation esprima/invalid-syntax/migrated_0239/input.js 1:29-1:31
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0239/input.js 1:14-1:35
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["static", RAW_MARKUP {value: " is a reserved word"}]}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0239/input.js>
				end: Position 1:29
				start: Position 1:23
			}
		}
	]
	directives: [
		JSDirective {
			value: "use strict"
			loc: SourceLocation esprima/invalid-syntax/migrated_0239/input.js 1:0-1:13
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0239/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0239/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0239/input.js:1:23 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ static is a reserved word

    "use strict"; function static() { }
                           ^^^^^^


```