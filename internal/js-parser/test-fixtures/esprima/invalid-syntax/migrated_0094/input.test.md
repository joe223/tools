# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0094`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSNumericLiteral {
					value: 0
					format: "octal"
					loc: SourceLocation esprima/invalid-syntax/migrated_0094/input.js 1:21-1:23
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "a"
							loc: SourceLocation esprima/invalid-syntax/migrated_0094/input.js 1:15-1:16 (a)
						}
					]
					loc: SourceLocation esprima/invalid-syntax/migrated_0094/input.js 1:14-1:20
				}
				loc: SourceLocation esprima/invalid-syntax/migrated_0094/input.js 1:14-1:23
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0094/input.js 1:14-1:23
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
				message: RAW_MARKUP {value: "Legacy octal literals are not allowed in strict mode"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0094/input.js>
				end: Position 1:23
				start: Position 1:23
			}
		}
	]
	directives: [
		JSDirective {
			value: "use strict"
			loc: SourceLocation esprima/invalid-syntax/migrated_0094/input.js 1:0-1:13
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0094/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0094/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0094/input.js:1:23 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Legacy octal literals are not allowed in strict mode

    "use strict"; (a) => 00
                           ^


```