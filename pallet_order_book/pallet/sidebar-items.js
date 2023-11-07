window.SIDEBAR_ITEMS = {"enum":[["Call","Contains one variant per dispatchable that can be called by an extrinsic."],["Error","Custom dispatch errors of this pallet."],["Event","The event emitted by this pallet."]],"struct":[["Order","Order Storage item. Contains fields relevant to order information"],["Pallet","The pallet implementing the on-chain logic."]],"trait":[["Config","Configuration trait of this pallet."]],"type":[["AssetPairOrders","Map of Vec containing OrderIds of same asset in/out pairs. Allows looking up orders available corresponding pairs."],["BalanceOf",""],["Module","Type alias to `Pallet`, to be used by `construct_runtime`."],["OrderIdNonceStore","Stores OrderIdNonce for orders placed Given that OrderIdNonce is to ensure that all orders have a unique ID, we can use just one OrderIdNonce, which means that we only have one val in storage, and we don’t have to insert new map values upon a new account/currency order creation."],["OrderOf","Order of pallet config type"],["Orders","Map of Orders to look up orders by their order id."],["TradingPair","Storage of valid order pairs. Stores:"],["UserOrders","Map of orders for a particular user Used to query orders for a particular user using the account id of the user as prefix"]]};