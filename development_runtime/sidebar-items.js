window.SIDEBAR_ITEMS = {"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is used to limit the maximal weight of a single extrinsic."],["CENTI_CFG",""],["CFG",""],["DAYS",""],["DEFAULT_FEE_VALUE","Value for a not specified fee key."],["GENERAL_CURRENCY_INDEX_PREFIX","Unhashed 36-bytes prefix for currencies managed by LiquidityPools."],["HOURS",""],["MAXIMUM_BLOCK_WEIGHT","We allow for 0.5 seconds of compute with a 6 second average block time."],["MAX_TOKEN_NAME_LENGTH_BYTES","The max length allowed for a tranche token name"],["MAX_TOKEN_SYMBOL_LENGTH_BYTES","The max length allowed for a tranche token symbol"],["MICRO_CFG",""],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MILLISECS_PER_DAY","Milliseconds per day"],["MILLI_CFG",""],["MINUTES",""],["MIN_VESTING","Minimum vesting amount, in CFG/AIR"],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by Operational  extrinsics."],["SAFE_XCM_VERSION","The safe XCM version of pallet-xcm, same as on relay chain"],["SECONDS_PER_DAY",""],["SECONDS_PER_HOUR",""],["SECONDS_PER_MINUTE",""],["SECONDS_PER_MONTH",""],["SECONDS_PER_WEEK",""],["SECONDS_PER_YEAR",""],["SLOT_DURATION",""],["TRANSACTION_RECOVERY_ID","Transaction recovery ID used for generating a signature in the Ethereum Transaction pallet. As per: https://github.com/PureStake/moonbeam/blob/fb63014a5e487f17e31283776e4f6b0befd009a2/primitives/xcm/src/ethereum_xcm.rs#L167"],["TREASURY_FEE_RATIO","% of fee addressed to the Treasury. The reminder % will be for the block author."],["VERSION","Runtime version."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["OriginCaller",""],["ProxyType","The type used to represent the kinds of proxying allowed."],["RuntimeCall",""],["RuntimeEvent",""],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["deposit",""],["native_version","Native version."]],"macro":[["production_or_benchmark",""]],"mod":[["account_conversion",""],["api",""],["apis","Runtime apis useful in the Centrifuge ecosystem"],["asset_registry","AssetRegistry’s AssetProcessor"],["changes",""],["currency_decimals",""],["evm",""],["fees",""],["foreign_investments",""],["gateway",""],["investment_portfolios","Module for investment portfolio common to all runtimes"],["liquidity_pools",""],["oracle",""],["origin",""],["xcm",""],["xcm_fees",""],["xcm_transactor",""]],"struct":[["AnnouncementDepositBase",""],["AnnouncementDepositFactor",""],["AttributeDepositBase",""],["BaseCallFilter","Base Call Filter"],["BasicDeposit",""],["BlockRewardCurrency",""],["Burn",""],["CandidacyBond",""],["ChainBridgePalletId",""],["ChainId",""],["ChallengeTime",""],["ClaimsPalletId",""],["CollatorGroupId",""],["CollectionDeposit",""],["CommitAnchorFeeKey",""],["CooloffPeriod",""],["CouncilMaxMembers",""],["CouncilMaxProposals",""],["CouncilMotionDuration",""],["CrowdloanClaimPalletId",""],["CrowdloanRewardPalletId",""],["CurrencyAdapter","Implements the transaction payment for a pallet implementing the `Currency` trait (eg. the pallet_balances) using an unbalance handler (implementing `OnUnbalanced`)."],["CurrencyED",""],["CurrencyHooks",""],["DefaultFeeValue",""],["DefaultKeyDeposit",""],["DefaultMaxNAVAge",""],["DefaultMinEpochTime",""],["DepositBase",""],["DepositFactor",""],["DepositPerByte",""],["DesiredMembers",""],["DesiredRunnersUp",""],["Editors",""],["ElectionsPhragmenModuleId",""],["EnactmentPeriod",""],["EthAddress","A generic representation of a local address. A resource id points to this. It may be a registry id (20 bytes) or a fungible asset type (in the future). Constrained to 32 bytes just as an upper bound to store efficiently."],["ExistentialDeposit",""],["FastTrackVotingPeriod",""],["FieldDeposit",""],["GenesisConfig",""],["InitialEpochDuration",""],["InstantAllowed",""],["IsTrancheInvestor","Checks whether the given `who` has the role of a `TrancheInvestor` for the given pool."],["ItemDeposit",""],["ItemId","A representation of ItemId for Uniques."],["LaunchPeriod",""],["Limit",""],["MaxActiveLoansPerPool",""],["MaxAdditionalFields",""],["MaxApprovals",""],["MaxAuthorities",""],["MaxCandidates",""],["MaxChangesPerEpoch",""],["MaxCollectionSize",""],["MaxCurrencyMovements",""],["MaxGroups",""],["MaxHasDispatchedSize",""],["MaxHrmpRelayFee",""],["MaxInstructions","Maximum number of instructions in a single XCM message."],["MaxInvulnerables",""],["MaxKeys",""],["MaxLocks",""],["MaxNAVAgeUpperBound",""],["MaxOutstandingCollects",""],["MaxPending",""],["MaxPoolsWithExternalPrices",""],["MaxPriceOracleMembers",""],["MaxProofLength",""],["MaxProposals",""],["MaxProxies",""],["MaxRateCount",""],["MaxRegistrars",""],["MaxReserves",""],["MaxRolesPerPool",""],["MaxScheduledPerBlock",""],["MaxSignatories",""],["MaxSizeMetadata",""],["MaxSubAccounts",""],["MaxTranches",""],["MaxVoters",""],["MaxVotes",""],["MaxWriteOffPolicySize",""],["MaximumBlockWeight",""],["MaximumSchedulerWeight",""],["MetadataDepositBase",""],["MigrationMaxAccounts",""],["MigrationMaxProxies",""],["MigrationMaxVestings",""],["MinCandidates",""],["MinDelay",""],["MinEpochTimeLowerBound",""],["MinEpochTimeUpperBound",""],["MinFulfillmentAmountNative",""],["MinUpdateDelay",""],["MinVestedTransfer",""],["MinimalPayoutAmount",""],["MinimumDeposit",""],["MinimumPeriod",""],["NativeToken",""],["NativeTokenId",""],["NativeTokenTransferFeeKey",""],["NftProofValidationFeeKey",""],["NftSalesPalletId",""],["NoPreimagePostponement",""],["Offset",""],["OperationalFeeMultiplier","This value increases the priority of `Operational` transactions by adding a “virtual tip” that’s equal to the `OperationalFeeMultiplier * final_fee`."],["OrderPairVecSize",""],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["Period",""],["PoolCurrency",""],["PoolDeposit",""],["PoolPalletId",""],["PoolPalletIndex","The index with which this pallet is instantiated in this runtime."],["PotId",""],["PreCommitDepositFeeKey",""],["PreimageBaseDeposit",""],["PreimageByteDeposit",""],["PreimageMaxSize",""],["ProposalBond",""],["ProposalBondMaximum",""],["ProposalBondMinimum",""],["ProposalLifetime",""],["ProxyDepositBase",""],["ProxyDepositFactor",""],["RegistryId","A representation of registryID."],["RelayerVoteThreshold",""],["ReservedDmpWeight",""],["ReservedXcmpWeight",""],["ResourceHashId",""],["RestrictedTokens",""],["RewardCurrency",""],["RewardsPalletId",""],["RootOperatorOraclePrice",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeBlockLength",""],["RuntimeBlockWeights",""],["RuntimeOrigin","The runtime origin type representing the origin of a call."],["SS58Prefix",""],["SessionKeys",""],["SessionLength",""],["SingleCurrencyMovement",""],["SpendPeriod",""],["StakeAmount",""],["SubAccountDeposit",""],["TargetedFeeAdjustment","A struct to update the weight multiplier per block. It implements `Convert<Multiplier, Multiplier>`, meaning that it can convert the previous multiplier to the next one. This should be called on `on_finalize` of a block, prior to potentially cleaning the weight data from the system pallet."],["TermDuration",""],["TokenId",""],["TrancheWeight","A representation of a tranche weight, used to weight importance of a tranche"],["TransactionByteFee","TransactionByteFee is set to 0.01 MicroCFG"],["TransactionConverter",""],["TransferAllowlistFeeKey",""],["TreasuryAccount",""],["TreasuryPalletId",""],["UncleGenerations",""],["UnitWeightCost","The amount of weight an XCM operation takes. This is a safe overestimate."],["UnvestedFundsAllowedWithdrawReasons",""],["UpdateGuard",""],["Version",""],["VotingBond",""],["VotingBondBase",""],["VotingPeriod",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","The type for looking up accounts. We don’t expect more than 4 billion of them, but you never know…"],["Address","The address format for describing accounts."],["AllOfCouncil","All council members must vote yes to create this origin."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Anchor",""],["Aura",""],["AuraConfig",""],["AuraExt",""],["AuraExtConfig",""],["AuraId","Aura consensus authority."],["Authorship",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["BaseFee",""],["BaseFeeConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["BlockNumber","An index to a block."],["BlockRewards",""],["BlockRewardsBase",""],["BlockRewardsBaseConfig",""],["BlockRewardsConfig",""],["Bridge",""],["BridgeConfig",""],["Bytes",""],["Bytes32",""],["ChainBridge",""],["CheckedExtrinsic","Extrinsic type that has already been checked."],["Claims",""],["CollatorAllowlist",""],["CollatorAllowlistConfig",""],["CollatorSelection",""],["CollatorSelectionConfig",""],["CollectionId","A representation of CollectionId for Uniques"],["Council",""],["CouncilCollective","The council"],["CouncilConfig",""],["CrowdloanClaim",""],["CrowdloanReward",""],["CumulusXcm",""],["Democracy",""],["DemocracyConfig",""],["DmpQueue",""],["EVM",""],["EVMChainId",""],["EVMChainIdConfig",""],["EVMConfig",""],["Elections",""],["ElectionsConfig",""],["EnsureRootOr",""],["Ethereum",""],["EthereumConfig",""],["EthereumTransaction",""],["Executive","Executive: handles dispatch to the various modules."],["Fees",""],["FeesConfig",""],["FixedArray",""],["ForeignInvestments",""],["GapRewardMechanism",""],["HalfOfCouncil","1/2 of all council members must vote yes to create this origin."],["Hash","A hash of some data used by the chain."],["Hashing","The hashing algorithm used by the chain"],["Header","Block header type as expected by this runtime."],["IBalance","IBalance is the signed version of the Balance for orml tokens"],["Identity",""],["Index","Index of a transaction in the chain."],["InterestAccrual",""],["Investments",""],["Keystore",""],["LiquidityPools",""],["LiquidityPoolsAxelarGateway",""],["LiquidityPoolsGateway",""],["LiquidityRewards",""],["LiquidityRewardsBase",""],["LiquidityRewardsBaseConfig",""],["LoanId","A representation of a loan identifier"],["Loans",""],["Migration",""],["Multiplier","Fee multiplier."],["Multisig",""],["NftSales",""],["Nfts",""],["OrderBook",""],["OrderId","OrderId type we use to identify order per epoch."],["OrmlAssetRegistry",""],["OrmlAssetRegistryConfig",""],["OrmlTokens",""],["OrmlTokensConfig",""],["OrmlXcm",""],["PalletIndex","The type for indexing pallets on a Substrate runtime"],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainSystem",""],["ParachainSystemConfig",""],["Permissions",""],["PolkadotXcm",""],["PolkadotXcmConfig",""],["PoolEpochId","EpochId type we use to identify epochs in our revolving pools"],["PoolId","PoolId type we use."],["PoolRegistry",""],["PoolSystem",""],["Preimage",""],["PriceCollector",""],["PriceOracle",""],["PriceOracleMembership",""],["Proxy",""],["RandomnessCollectiveFlip",""],["Salt",""],["Scheduler",""],["Session",""],["SessionConfig",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["Sudo",""],["SudoConfig",""],["System",""],["SystemConfig",""],["ThreeFourthOfCouncil","3/4 of all council members must vote yes to create this origin."],["Timestamp",""],["Tokens",""],["TrancheId","A representation of a tranche identifier"],["TransactionPayment",""],["TransferAllowList",""],["Treasury",""],["TreasuryConfig",""],["TwoThirdOfCouncil","2/3 of all council members must vote yes to create this origin."],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Uniques",""],["Utility",""],["Vesting",""],["VestingConfig",""],["XTokens",""],["XcmTransactor",""],["XcmWeigher","Xcm Weigher shared between multiple Xcm-related configs."],["XcmpQueue",""]]};