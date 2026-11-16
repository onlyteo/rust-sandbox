import React, {FC, ReactElement} from "react";

export const NotFound: FC = (): ReactElement => {
    return (
        <div className="row">
            <div className="col">
                <h2 className="emphasized-text">Not Found</h2>
            </div>
        </div>
    );
};
