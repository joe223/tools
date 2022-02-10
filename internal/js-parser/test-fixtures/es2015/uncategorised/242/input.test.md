# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 242`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSNumericLiteral {
					value: 42
					loc: SourceLocation es2015/uncategorised/242/input.js 1:29-1:31
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingAssignmentPattern {
							operator: "="
							left: JSBindingIdentifier {
								name: "eval"
								loc: SourceLocation es2015/uncategorised/242/input.js 1:15-1:19 (eval)
							}
							right: JSNumericLiteral {
								value: 10
								loc: SourceLocation es2015/uncategorised/242/input.js 1:22-1:24
							}
							loc: SourceLocation es2015/uncategorised/242/input.js 1:15-1:24
						}
					]
					loc: SourceLocation es2015/uncategorised/242/input.js 1:14-1:28
				}
				loc: SourceLocation es2015/uncategorised/242/input.js 1:14-1:31
			}
			loc: SourceLocation es2015/uncategorised/242/input.js 1:14-1:31
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["eval", RAW_MARKUP {value: " is a reserved word"}]}
			location: {
				language: "js"
				path: UIDPath<es2015/uncategorised/242/input.js>
				end: Position 1:19
				start: Position 1:15
			}
		}
	]
	directives: [
		JSDirective {
			value: "use strict"
			loc: SourceLocation es2015/uncategorised/242/input.js 1:0-1:13
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/242/input.js>
	loc: SourceLocation es2015/uncategorised/242/input.js 1:0-1:31
}
```

### `diagnostics`

```

 es2015/uncategorised/242/input.js:1:15 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ eval is a reserved word

    "use strict"; (eval = 10) => 42
                   ^^^^


```