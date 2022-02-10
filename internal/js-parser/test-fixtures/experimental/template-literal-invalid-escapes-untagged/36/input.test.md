# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > template-literal-invalid-escapes-untagged > 36`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSTemplateLiteral {
				expressions: [
					JSNumericLiteral {
						value: 0
						loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:7-1:8
					}
					JSNumericLiteral {
						value: 1
						loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:15-1:16
					}
				]
				quasis: [
					JSTemplateElement {
						cooked: "left"
						raw: "left"
						tail: false
						loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:1-1:5
					}
					JSTemplateElement {
						cooked: "\\u{}"
						raw: "\\u{}"
						tail: false
						loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:9-1:13
					}
					JSTemplateElement {
						cooked: "right"
						raw: "right"
						tail: true
						loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:17-1:22
					}
				]
				loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:0-1:23
			}
			loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:0-1:23
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
				message: RAW_MARKUP {value: "Invalid escape sequence in template"}
			}
			location: {
				language: "js"
				path: UIDPath<experimental/template-literal-invalid-escapes-untagged/36/input.js>
				end: Position 1:10
				start: Position 1:10
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/template-literal-invalid-escapes-untagged/36/input.js>
	loc: SourceLocation experimental/template-literal-invalid-escapes-untagged/36/input.js 1:0-1:23
}
```

### `diagnostics`

```

 experimental/template-literal-invalid-escapes-untagged/36/input.js:1:10 parse(js) ━━━━━━━━━━━━━━━━━

  ✖ Invalid escape sequence in template

    `left${0}\u{}${1}right`
              ^


```