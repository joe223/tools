# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 249`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSNumericLiteral {
					value: 0
					format: "octal"
					loc: SourceLocation es2015/uncategorised/249/input.js 1:21-1:23
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "a"
							loc: SourceLocation es2015/uncategorised/249/input.js 1:15-1:16 (a)
						}
					]
					loc: SourceLocation es2015/uncategorised/249/input.js 1:14-1:20
				}
				loc: SourceLocation es2015/uncategorised/249/input.js 1:14-1:23
			}
			loc: SourceLocation es2015/uncategorised/249/input.js 1:14-1:23
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
				path: UIDPath<es2015/uncategorised/249/input.js>
				end: Position 1:23
				start: Position 1:23
			}
		}
	]
	directives: [
		JSDirective {
			value: "use strict"
			loc: SourceLocation es2015/uncategorised/249/input.js 1:0-1:13
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/249/input.js>
	loc: SourceLocation es2015/uncategorised/249/input.js 1:0-1:23
}
```

### `diagnostics`

```

 es2015/uncategorised/249/input.js:1:23 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Legacy octal literals are not allowed in strict mode

    "use strict"; (a) => 00
                           ^


```