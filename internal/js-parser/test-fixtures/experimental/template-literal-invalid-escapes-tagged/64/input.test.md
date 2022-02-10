# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > template-literal-invalid-escapes-tagged > 64`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSTaggedTemplateExpression {
				quasi: JSTemplateLiteral {
					expressions: [
						JSNumericLiteral {
							value: 0
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:16-1:17
						}
						JSNumericLiteral {
							value: 1
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:28-1:29
						}
					]
					quasis: [
						JSTemplateElement {
							cooked: "left"
							raw: "left"
							tail: false
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:10-1:14
						}
						JSTemplateElement {
							cooked: "\\u{\\u{0}"
							raw: "\\u{\\u{0}"
							tail: false
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:18-1:26
						}
						JSTemplateElement {
							cooked: "right"
							raw: "right"
							tail: true
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:30-1:35
						}
					]
					loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:9-1:36
				}
				tag: JSReferenceIdentifier {
					name: "sampleTag"
					loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:0-1:9 (sampleTag)
				}
				loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:0-1:36
			}
			loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:0-1:36
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/template-literal-invalid-escapes-tagged/64/input.js>
	loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/64/input.js 1:0-1:36
}
```

### `diagnostics`

```

```