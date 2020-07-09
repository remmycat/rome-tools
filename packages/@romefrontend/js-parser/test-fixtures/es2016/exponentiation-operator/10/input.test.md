# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `es2016 > exponentiation-operator > 10`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "input.js"
		end: Object {
			column: 0
			index: 9
			line: 2
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				advice: Array []
				category: "parse/js"
				message: PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE {value: "Illegal expression. Wrap left hand side or entire exponentiation in parentheses."}
			}
			location: Object {
				filename: "input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 2
					index: 2
					line: 1
				}
				start: Object {
					column: 1
					index: 1
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "input.js"
				end: Object {
					column: 8
					index: 8
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			expression: JSBinaryExpression {
				operator: "**"
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 7
						index: 7
						line: 1
					}
					start: Object {
						column: 0
						index: 0
						line: 1
					}
				}
				right: JSNumericLiteral {
					value: 6
					format: undefined
					loc: Object {
						filename: "input.js"
						end: Object {
							column: 7
							index: 7
							line: 1
						}
						start: Object {
							column: 6
							index: 6
							line: 1
						}
					}
				}
				left: JSUnaryExpression {
					operator: "-"
					prefix: true
					loc: Object {
						filename: "input.js"
						end: Object {
							column: 2
							index: 2
							line: 1
						}
						start: Object {
							column: 0
							index: 0
							line: 1
						}
					}
					argument: JSNumericLiteral {
						value: 5
						format: undefined
						loc: Object {
							filename: "input.js"
							end: Object {
								column: 2
								index: 2
								line: 1
							}
							start: Object {
								column: 1
								index: 1
								line: 1
							}
						}
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```

 input.js:1:1 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Illegal expression. Wrap left hand side or entire exponentiation in parentheses.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```