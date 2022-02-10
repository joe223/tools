# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-yield > invalid-yield-strict-arrow-parameter-default`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:29-1:31
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingAssignmentPattern {
							operator: "="
							left: JSBindingIdentifier {
								name: "x"
								loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:15-1:16 (x)
							}
							right: JSReferenceIdentifier {
								name: "yield"
								loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:19-1:24 (yield)
							}
							loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:15-1:24
						}
					]
					loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:14-1:28
				}
				loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:14-1:31
			}
			loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:14-1:31
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["yield", RAW_MARKUP {value: " is a reserved word"}]}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js>
				end: Position 1:24
				start: Position 1:19
			}
		}
	]
	directives: [
		JSDirective {
			value: "use strict"
			loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:0-1:13
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js>
	loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-yield/invalid-yield-strict-arrow-parameter-default/input.js:1:19 parse(js) ━━━━━━━━━

  ✖ yield is a reserved word

    "use strict"; (x = yield) => {}
                       ^^^^^


```