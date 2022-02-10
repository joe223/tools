# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > invalid-escape-sequence-function-list`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "async"
				loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:1-1:11 (async)
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:0-1:11
		}
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: ""
				loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:20-1:21 ()
			}
			body: JSBlockStatement {
				body: [
					JSExpressionStatement {
						expression: JSReferenceIdentifier {
							name: "await"
							loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:25-1:30 (await)
						}
						loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:25-1:30
					}
					JSExpressionStatement {
						expression: JSReferenceIdentifier {
							name: "x"
							loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:31-1:32 (x)
						}
						loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:31-1:32
					}
				]
				directives: []
				loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:23-1:34
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:21-1:22
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:12-1:34
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:34-1:35
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:34-1:35
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: [log {category: "info", text: [RAW_MARKUP {value: "Expected character <emphasis>"}, ",", RAW_MARKUP {value: "</emphasis>"}]}]
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, "f", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2017/async-functions/invalid-escape-sequence-function-list/input.js>
				end: Position 1:20
				start: Position 1:12
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/invalid-escape-sequence-function-list/input.js>
	loc: SourceLocation es2017/async-functions/invalid-escape-sequence-function-list/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2017/async-functions/invalid-escape-sequence-function-list/input.js:1:12 parse(js) ━━━━━━━━━━━━━━

  ✖ Unexpected character f

    (\u0061sync function() { await x })
                ^^^^^^^^

  ℹ Expected character ,


```