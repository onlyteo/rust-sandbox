import React, {FC, ReactElement} from "react";
import {OrderStatus} from "../types";

export const StatusBadge: FC<{ status: OrderStatus }> = ({status}): ReactElement => (
    <span className={`badge ${status === "COMPLETED" ? "bg-success" : "bg-secondary"}`}>{status}</span>
);
