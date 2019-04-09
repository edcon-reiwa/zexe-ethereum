import React, { Component } from "react"

const ZEXE_ETH_LOCAL_STORAGE = "zexe_eth_edcon_local_storage"

class AccountComponent extends Component {

  /*
   TODO

   1. integrate local storage to keep track of transactions [DONE]
   2. calculate balance [DONE]
   3. display transaction history [DONE]
   4. integrate rust transaction generator
   5. integrate IPFS
   6. integrate smart contract [DONE]
   7. implement event watcher [DONE]

   */

  componentDidMount() {
    this.loadFromStorage()
  }

  constructor(props) {
    super(props)
    this.calculateBalance = this.calculateBalance.bind(this)
    this.mint = this.mint.bind(this)
    this.generateTx = this.generateTx.bind(this)
    this.handleChange = this.handleChange.bind(this)
    this.renderCommitmentHistory = this.renderCommitmentHistory.bind(this)
    this.state = {
      txValue: 0,
      serialNumbers: null,
      newCommitments: null,
      memo: null,
      digest: null,
      commitments: {}
    }
  }

  loadFromStorage() {
    let storage = JSON.parse(localStorage.getItem(ZEXE_ETH_LOCAL_STORAGE))
    if (!storage) {
      return
    }
    let userStorage = storage[this.props.accounts[0]]
    if (!userStorage) {
      return
    }

    this.setState({commitments: userStorage})
  }

  calculateBalance() {
    let balance = 0
    for (var i in this.state.commitments) {
      let commitment = this.state.commitments[i]
      if (!commitment.spent) {
        balance += parseInt(commitment.value)
      }
    }
    return balance
  }

  addCommitment(contentHash, value) {
    this.state.commitments[contentHash] = {
      value: value,
      spent: false
    }

    this.setState({commitments: this.state.commitments})

    let storage = JSON.parse(localStorage.getItem(ZEXE_ETH_LOCAL_STORAGE))
    if (!storage) {
      storage = {}
    }
    let userStorage = storage[this.props.accounts[0]]
    if (!userStorage) {
      storage[this.props.accounts[0]] = {}
    }

    storage[this.props.accounts[0]] = this.state.commitments

    localStorage.setItem(ZEXE_ETH_LOCAL_STORAGE, JSON.stringify(storage))
  }

  handleChange(event) {
    this.setState({
      [event.target.name]: event.target.value
    })
  }

  generateTx(e) {
    e.preventDefault()

    alert("Not yet implemented. Please generate transaction locally")
  }

  mint(e) {
    e.preventDefault()

    if (
        !this.state.serialNumbers ||
        !this.state.newCommitments ||
        !this.state.memo ||
        !this.state.digest ||
        this.state.txValue <= 0
    ) {
      alert("Please check inputs")
      return
    }

    let serialNumbers = this.state.serialNumbers.split(",").map(x => x.trim())
    let newCommitments = this.state.newCommitments.split(",").map(x => x.trim())
    let memo = this.state.memo.split(",").map(x => x.trim())
    let digest = this.state.digest.trim()

    this.props.contract.methods.Mint(
      serialNumbers, newCommitments, memo, digest
    ).send({
      from: this.props.accounts[0],
      gasPrice: this.props.web3.utils.toWei("1", "gwei")
    }).then(result => {
      let events = result.events.TransferHash
      if (!Array.isArray(events)) {
        events = [events]
      }

      for (var i in events) {
        this.addCommitment(events[i].returnValues[1], this.state.txValue)
      }
    })
  }

  renderCommitment(commitment) {
    return (
      <tr key={commitment[0]}>
        <td className="id-column">{commitment[0]}</td>
        <td className="value-column">{commitment[1]}</td>
        <td className="spent-column">{commitment[2] ? "✓" : ""}</td>
      </tr>
    )
  }

  renderCommitmentHistory() {
    var commitmentsList = []

    for(var key in this.state.commitments) {
      let commitment = this.state.commitments[key]
      commitmentsList.push([key, commitment.value, commitment.spent])
    }
    return commitmentsList.map(commitment => this.renderCommitment(commitment))
  }

  render(props) {
    return (
      <div className="account-section">
        <h3 className="account-title">Account Info</h3>
        <div className="pure-g account-top-section">
          <div className="pure-u-1 pure-u-lg-1-2">
            <h4>
              <span className="account-key">Address: </span>
              {this.props.accounts[0]}
            </h4>
            <h4>
              <span className="account-key">Balance: </span>
              {this.calculateBalance()} RWA
            </h4>
            <h4 className="account-key send-key">
              <span>Transaction Generation: </span>
            </h4>
            <form className="pure-form pure-form-stacked send-form">
              <fieldset>
                <input type="text" placeholder="To:" />
                <input type="text" placeholder="Value:" />
                <button
                  type="submit"
                  className="pure-button pure-button-primary"
                  onClick={this.generateTx}
                >
                  Generate transaction
                </button>
              </fieldset>
            </form>
          </div>
          <div className="pure-u-1 pure-u-lg-1-2">
            <h4 className="account-key send-key">
              <span>Minting: </span>
            </h4>
            <form className="pure-form pure-form-stacked send-form">
              <fieldset>
                <input type="number" name="txValue" placeholder="Value:" onChange={this.handleChange} />
                <input type="text" name="serialNumbers" placeholder="serial numbers (bytes32[])" onChange={this.handleChange} />
                <input type="text" name="newCommitments" placeholder="new commitments (bytes32[])" onChange={this.handleChange} />
                <input type="text" name="memo" placeholder="memo (bytes32[])" onChange={this.handleChange} />
                <input type="text" name="digest" placeholder="digest (bytes32)" onChange={this.handleChange} />
                <button
                  type="submit"
                  className="pure-button pure-button-primary"
                  onClick={this.mint}
                >
                  Transfer
                </button>
              </fieldset>
            </form>
            <h4 className="account-key send-key">
              <span>Transfering: <i>coming soon</i></span>
            </h4>
          </div>
        </div>

        <h4>
          <span className="account-key">History:</span>
          <table className="pure-table transaction-table">
            <thead>
              <tr>
                <th>id</th>
                <th>Value</th>
                <th>Spent</th>
              </tr>
            </thead>

            <tbody>
              {this.renderCommitmentHistory()}
            </tbody>
        </table>
        </h4>
      </div>
    )
  }
}

export default AccountComponent
