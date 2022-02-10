# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2019 > json-strings > template-line-separator`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			trailingComments: ["0"]
			expression: JSTemplateLiteral {
				expressions: []
				quasis: [
					JSTemplateElement {
						cooked: "before\u2028after"
						raw: "before\u2028after"
						tail: true
						loc: SourceLocation es2019/json-strings/template-line-separator/input.js 1:2-2:5
					}
				]
				loc: SourceLocation es2019/json-strings/template-line-separator/input.js 1:1-2:6
			}
			loc: SourceLocation es2019/json-strings/template-line-separator/input.js 1:0-2:8
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: "      ^ That's a U+2028 LINE SEPARATOR UTF-16 char (between 'before' and 'after')"
			loc: SourceLocation es2019/json-strings/template-line-separator/input.js 3:0-3:83
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2019/json-strings/template-line-separator/input.js>
	loc: SourceLocation es2019/json-strings/template-line-separator/input.js 1:0-4:0
}
```

### `diagnostics`

```

```