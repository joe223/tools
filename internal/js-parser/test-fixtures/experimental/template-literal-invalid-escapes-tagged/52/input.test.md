# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > template-literal-invalid-escapes-tagged > 52`

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
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:16-1:17
						}
						JSNumericLiteral {
							value: 1
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:25-1:26
						}
					]
					quasis: [
						JSTemplateElement {
							cooked: "left"
							raw: "left"
							tail: false
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:10-1:14
						}
						JSTemplateElement {
							cooked: "\\u{\\\\"
							raw: "\\u{\\\\"
							tail: false
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:18-1:23
						}
						JSTemplateElement {
							cooked: "right"
							raw: "right"
							tail: true
							loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:27-1:32
						}
					]
					loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:9-1:33
				}
				tag: JSReferenceIdentifier {
					name: "sampleTag"
					loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:0-1:9 (sampleTag)
				}
				loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:0-1:33
			}
			loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:0-1:33
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/template-literal-invalid-escapes-tagged/52/input.js>
	loc: SourceLocation experimental/template-literal-invalid-escapes-tagged/52/input.js 1:0-1:33
}
```

### `diagnostics`

```

```