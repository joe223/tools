# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > invalid-parens-async-arrow`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSCallExpression {
				arguments: [
					JSReferenceIdentifier {
						name: "a"
						loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:8-1:9 (a)
					}
				]
				callee: JSReferenceIdentifier {
					name: "async"
					loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:1-1:6 (async)
				}
				loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:0-1:10
			}
			loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:0-1:10
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:11-1:13
			}
			loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:11-1:13
		}
		JSBlockStatement {
			body: []
			directives: []
			loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:14-1:16
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
				path: UIDPath<es2017/async-functions/invalid-parens-async-arrow/input.js>
				end: Position 1:10
				start: Position 1:11
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/invalid-parens-async-arrow/input.js>
	loc: SourceLocation es2017/async-functions/invalid-parens-async-arrow/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2017/async-functions/invalid-parens-async-arrow/input.js:1:11 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    (async)(a) => {}
               ^


```