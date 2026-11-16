import React, {FC, ReactElement} from "react";
import {Alert} from "../types";

export const AlertMessage: FC<{ alert?: Alert }> = ({alert}): ReactElement => {
    if (!alert) {
        return <></>;
    }
    return (
        <div className="row mt-4">
            <div className="col">
                <div className={`alert alert-${alert.variant}`} role="alert">{alert.message}</div>
            </div>
        </div>
    );
};
