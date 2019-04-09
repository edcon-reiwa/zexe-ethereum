import React, { Component } from "react"



class AccountComponent extends Component {

  /*
   TODO

   1. integrate local storage to keep track of transactions
   2. calculate balance
   3. display transaction history
   4. integrate rust transaction generator
   5. integrate IPFS
   6. integrate smart contract
   7. implement event watcher

   */

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
              XX RWA
            </h4>
          </div>
          <div className="pure-u-1 pure-u-lg-1-2">
            <h4 className="account-key send-key">
              <span>Send: </span>
            </h4>
            <form className="pure-form pure-form-stacked send-form">
              <fieldset>
                <input type="text" className="recipient-box" placeholder="To:" />
                <input type="text" placeholder="Value" />
                <button type="submit" class="pure-button pure-button-primary">
                  Transfer
                </button>
              </fieldset>
            </form>
          </div>
        </div>

        <h4>
          <span className="account-key">History:</span>
        </h4>
      </div>
    )
  }
}

export default AccountComponent
