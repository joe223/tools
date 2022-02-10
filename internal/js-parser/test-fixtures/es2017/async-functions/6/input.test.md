# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > 6`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation es2017/async-functions/6/input.js 1:12-1:14
				}
				head: JSFunctionHead {
					async: true
					hasHoistedVars: false
					params: []
					loc: SourceLocation es2017/async-functions/6/input.js 1:0-1:11
				}
				loc: SourceLocation es2017/async-functions/6/input.js 1:0-1:14
			}
			loc: SourceLocation es2017/async-functions/6/input.js 1:0-1:14
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation es2017/async-functions/6/input.js 1:15-1:15
			}
			loc: SourceLocation es2017/async-functions/6/input.js 1:14-1:16
		}
	]
	comments: []
	corrupt: true
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
				path: UIDPath<es2017/async-functions/6/input.js>
				end: Position 1:14
				start: Position 1:14
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/6/input.js>
	loc: SourceLocation es2017/async-functions/6/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2017/async-functions/6/input.js:1:14 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    async () => {}()
                  ^


```