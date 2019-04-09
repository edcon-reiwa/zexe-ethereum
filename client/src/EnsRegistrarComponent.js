import React, { Component } from "react"

class EnsRegistrarComponent extends Component {

  constructor(props) {
    super(props)
    this.handleChange = this.handleChange.bind(this)
    this.register = this.register.bind(this)
    this.state = {
      address: null,
      ensDomain: null,
    }
  }

  handleChange(event) {
    this.setState({
      [event.target.name]: event.target.value
    })
  }

  register(e) {
    e.preventDefault()

    if (!this.state.address || !this.state.ensDomain) {
      alert("Please check inputs")
      return
    }

    this.props.contract.methods.register(
      this.props.web3.utils.sha3(this.state.ensDomain),
      this.props.accounts[0]
    ).send({
      from: this.props.accounts[0],
    }).then(result => {
      alert("registered!")
      console.log(result)
    }).catch(error => {
      alert("error registering")
      console.log(error)
    })
  }

  render() {
    return (
      <div className="ens-section">
        <h3 className="account-title">ENS Management</h3>
        <form className="pure-form pure-form-stacked ens-form">
          <fieldset>
            <input type="text" name="address" placeholder="Address" onChange={this.handleChange} />
            <input type="text" name="ensDomain" placeholder="ENS Domain" onChange={this.handleChange} />
            <button
              type="submit"
              className="pure-button pure-button-primary"
              onClick={this.register}
            >
              Register Address
            </button>
          </fieldset>
        </form>
      </div>
    )
  }
}

export default EnsRegistrarComponent
