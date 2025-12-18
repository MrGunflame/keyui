import { invoke } from '@tauri-apps/api/core';

export class Client {
    async send(method: string, payload: any): Promise<any> {
        let resp: any = await invoke("send_msg", {
            "method": method,
            "params": payload,
        });

        console.log(resp);

        if (resp.error) {
            let err = new ApiError();
            err.code = resp.error.code;
            err.data = resp.error.data;
            err.message = resp.error.message;
            throw err;
        }

        return resp.result;
    }

    public async version(): Promise<string> {
        return await this.send("meta/version", null);
    }

    public async loadKey(content: string): Promise<ProofId> {
        return await this.send("loading/loadKey", content);
    }

    public async proofTreeRoot(proof: ProofId): Promise<TreeNodeDesc> {
        return await this.send("proofTree/root", proof);
    }

    public async goalPrint(id: NodeId, options: PrintOptions): Promise<NodeTextDesc> {
        return await this.send("goal/print", [id, options]);
    }
}

class ApiError {
    code: number = 0;
    data: string = "";
    message: string = "";

    public toString(): string {
        return `${this.message} (Code ${this.code}):\n${this.data}`;
    }
}

export type ProofId = {
    env: EnvId;
    proofId: string,
};

export type EnvId = {
    envId: string,
};

export type NodeId = {
    nodeId: number;
    proofId: ProofId;
};

export type TreeNodeDesc = {
    id: NodeId;
    name: string;
};

export type PrintOptions = {
    unicode: boolean,
    width: number,
    indentation: number,
    pure: boolean,
    termLabels: boolean,
};

export type NodeTextDesc = {
    id: NodeTextId,
    result: string,
};

export type NodeTextId = {
    nodeId: NodeId,
    nodeTextId: number,
};
