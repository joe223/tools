# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > categorized > invalid-assignment-pattern-4`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentArrayPattern {
					elements: [
						JSAssignmentObjectPattern {
							properties: [
								JSAssignmentObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "b"
											loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:2-1:3 (b)
										}
										loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:2-1:3
									}
									value: JSAssignmentArrayPattern {
										elements: [
											JSAssignmentArrayPattern {
												elements: [
													JSAssignmentAssignmentPattern {
														operator: "="
														left: JSAssignmentIdentifier {
															name: "a"
															loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:8-1:9 (a)
														}
														right: JSNumericLiteral {
															value: 1
															loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:12-1:13
														}
														loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:8-1:13
													}
												]
												loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:7-1:14
											}
										]
										loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:5-1:16
									}
									loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:2-1:16
								}
							]
							loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:1-1:17
						}
					]
					loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:0-1:18
				}
				right: JSReferenceIdentifier {
					name: "t"
					loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:21-1:22 (t)
				}
				loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:0-1:22
			}
			loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:0-1:22
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/categorized/invalid-assignment-pattern-4/input.js>
	loc: SourceLocation core/categorized/invalid-assignment-pattern-4/input.js 1:0-1:23
}
```

### `diagnostics`

```

```