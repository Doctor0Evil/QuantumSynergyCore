// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface ISGCToken {
    function transfer(address to, uint256 value) external returns (bool);
}

contract SGCTreasury {
    ISGCToken public immutable sgc;
    address public dao;
    uint256 public constant LIQUIDITY_POOL_SHARE = 40;
    uint256 public constant STAKING_SHARE = 30;
    uint256 public constant BONDS_SHARE = 20;
    uint256 public constant RESERVE_SHARE = 10;

    event Allocated(
        uint256 total,
        uint256 liquidity,
        uint256 staking,
        uint256 bonds,
        uint256 reserve
    );

    modifier onlyDao() {
        require(msg.sender == dao, "not dao");
        _;
    }

    constructor(address sgcAddress, address daoAddress) {
        sgc = ISGCToken(sgcAddress);
        dao = daoAddress;
    }

    function setDao(address newDao) external onlyDao {
        dao = newDao;
    }

    function allocate(uint256 amount, address lp, address staking, address bonds, address reserve)
        external
        onlyDao
    {
        uint256 liquidityAmount = (amount * LIQUIDITY_POOL_SHARE) / 100;
        uint256 stakingAmount = (amount * STAKING_SHARE) / 100;
        uint256 bondsAmount = (amount * BONDS_SHARE) / 100;
        uint256 reserveAmount = (amount * RESERVE_SHARE) / 100;

        require(sgc.transfer(lp, liquidityAmount), "lp transfer failed");
        require(sgc.transfer(staking, stakingAmount), "staking transfer failed");
        require(sgc.transfer(bonds, bondsAmount), "bonds transfer failed");
        require(sgc.transfer(reserve, reserveAmount), "reserve transfer failed");

        emit Allocated(
            amount,
            liquidityAmount,
            stakingAmount,
            bondsAmount,
            reserveAmount
        );
    }
}
