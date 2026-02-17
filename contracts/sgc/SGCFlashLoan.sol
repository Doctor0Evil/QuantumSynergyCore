// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface ISGC {
    function balanceOf(address) external view returns (uint256);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value)
        external
        returns (bool);
}

interface IFlashBorrower {
    function onFlashLoan(
        address initiator,
        uint256 amount,
        uint256 fee,
        bytes calldata data
    ) external;
}

contract SGCFlashLoan {
    ISGC public immutable sgc;
    uint256 public maxLoan;
    uint256 public feeBasisPoints; // 9 = 0.09%

    event FlashLoanExecuted(
        address indexed borrower,
        uint256 amount,
        uint256 fee
    );

    constructor(address sgcAddress, uint256 _maxLoan, uint256 _feeBps) {
        sgc = ISGC(sgcAddress);
        maxLoan = _maxLoan;
        feeBasisPoints = _feeBps;
    }

    function flashLoan(
        address borrower,
        uint256 amount,
        bytes calldata data
    ) external {
        require(amount <= maxLoan, "exceeds max loan");
        uint256 balanceBefore = sgc.balanceOf(address(this));
        require(balanceBefore >= amount, "insufficient liquidity");

        uint256 fee = (amount * feeBasisPoints) / 10000;
        require(fee > 0, "fee zero");

        require(sgc.transfer(borrower, amount), "transfer failed");

        IFlashBorrower(borrower).onFlashLoan(
            msg.sender,
            amount,
            fee,
            data
        );

        uint256 balanceAfter = sgc.balanceOf(address(this));
        require(
            balanceAfter >= balanceBefore + fee,
            "flash loan not repaid"
        );

        emit FlashLoanExecuted(borrower, amount, fee);
    }
}
