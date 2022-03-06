describe('Simulator interface', () => {
  it('meets Example A requirements', () => {
    cy.visit('http://localhost:3000')

    const commands = ['PLACE 0,0,NORTH', 'MOVE', 'REPORT'].join('\n')
    cy.get('textarea').type(commands)
    cy.contains('Evaluate').click()

    cy.contains('0,1,NORTH').should('exist')
  })

  it('meets Example B requirements', () => {
    cy.visit('http://localhost:3000')

    const commands = ['PLACE 0,0,NORTH', 'LEFT', 'REPORT'].join('\n')
    cy.get('textarea').type(commands)
    cy.contains('Evaluate').click()

    cy.contains('0,0,WEST').should('exist')
  })

  it('meets Example C requirements', () => {
    cy.visit('http://localhost:3000')

    const commands = ['PLACE 1,2,EAST', 'MOVE', 'MOVE', 'LEFT', 'MOVE', 'REPORT'].join('\n')
    cy.get('textarea').type(commands)
    cy.contains('Evaluate').click()

    cy.contains('3,3,NORTH').should('exist')
  })
})
