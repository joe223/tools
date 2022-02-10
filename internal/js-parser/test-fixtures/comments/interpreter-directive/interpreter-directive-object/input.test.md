# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `comments > interpreter-directive > interpreter-directive-object`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingObjectPattern {
							properties: [
								JSBindingObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "spawn"
											loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:6-3:11 (spawn)
										}
										loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:6-3:11
									}
									value: JSBindingIdentifier {
										name: "spawn"
										loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:6-3:11 (spawn)
									}
									loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:6-3:11
								}
							]
							loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:4-3:13
						}
						init: JSReferenceIdentifier {
							name: "x"
							loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:16-3:17 (x)
						}
						loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:4-3:17
					}
				]
				loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:0-3:18
			}
			loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 3:0-3:18
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	interpreter: JSInterpreterDirective {
		value: "/usr/bin/env babel-node"
		loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 1:1-1:25
	}
	path: UIDPath<comments/interpreter-directive/interpreter-directive-object/input.js>
	loc: SourceLocation comments/interpreter-directive/interpreter-directive-object/input.js 1:0-4:0
}
```

### `diagnostics`

```

```