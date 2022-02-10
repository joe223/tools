# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-arrow-function > non-arrow-param-followed-by-rest`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSNumericLiteral {
					value: 0
					loc: SourceLocation esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js 1:14-1:15
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "a"
							loc: SourceLocation esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js 1:2-1:3 (a)
						}
					]
					rest: JSBindingIdentifier {
						name: "b"
						loc: SourceLocation esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js 1:8-1:9 (b)
					}
					loc: SourceLocation esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js 1:0-1:13
				}
				loc: SourceLocation esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js 1:0-1:15
			}
			loc: SourceLocation esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js 1:0-1:16
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
				message: RAW_MARKUP {value: "Function parameters can't be parenthesized"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js>
				end: Position 1:3
				start: Position 1:2
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js>
	loc: SourceLocation esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-arrow-function/non-arrow-param-followed-by-rest/input.js:1:2 parse(js) ━━━━━━━━━━━━━

  ✖ Function parameters can't be parenthesized

    ((a),...b) => 0;
      ^


```