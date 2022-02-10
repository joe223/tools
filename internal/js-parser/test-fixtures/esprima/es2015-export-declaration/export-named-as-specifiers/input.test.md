# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-export-declaration > export-named-as-specifiers`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			specifiers: [
				JSExportLocalSpecifier {
					exported: JSIdentifier {
						name: "default"
						loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:15-1:22 (default)
					}
					local: JSReferenceIdentifier {
						name: "foo"
						loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:8-1:11 (foo)
					}
					loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:8-1:22
				}
				JSExportLocalSpecifier {
					exported: JSIdentifier {
						name: "bar"
						loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:24-1:27 (bar)
					}
					local: JSReferenceIdentifier {
						name: "bar"
						loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:24-1:27 (bar)
					}
					loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:24-1:27
				}
			]
			loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:0-1:29
		}
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "foo"
							loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 2:4-2:7 (foo)
						}
						loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 2:4-2:7
					}
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "bar"
							loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 2:9-2:12 (bar)
						}
						loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 2:9-2:12
					}
				]
				loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 2:0-2:13
			}
			loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 2:0-2:13
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "module"
	syntax: []
	path: UIDPath<esprima/es2015-export-declaration/export-named-as-specifiers/input.js>
	loc: SourceLocation esprima/es2015-export-declaration/export-named-as-specifiers/input.js 1:0-3:0
}
```

### `diagnostics`

```

```