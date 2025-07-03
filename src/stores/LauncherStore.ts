import { defineStore } from "pinia";
import { isInstancesVecResponse, validateBackendResponse, type Instance } from "../types";
import { getInstances, SaveInstance } from "../api";
import z from "zod/v4";

export const useLauncherStore = defineStore("launcher", {
	state: () => ({
		CurrentInstance: null as Instance | null,
		Instances: [] as Instance[],
		isAddInstanceModalOpen: false,
		currentView: 'welcome' as 'welcome' | 'instance' | 'settings',
		previousView: 'welcome' as 'welcome' | 'instance' | 'settings',
	}),
	actions: {
		async addInstance(instance: Instance) {
			let result = await SaveInstance(instance)
			if (!result.success) {
				console.log(result)
				return result;
			} else {
				this.Instances.push(instance);
				return result;
			}
		},
		setCurrentInstance(instance: Instance) {
			this.previousView = this.currentView;
			this.CurrentInstance = instance;
			this.currentView = 'instance';
		},
		navigateToSettings() {
			this.previousView = this.currentView;
			this.currentView = 'settings';
		},
		navigateToWelcome() {
			this.previousView = this.currentView;
			this.currentView = 'welcome';
			this.CurrentInstance = null;
		},
		goBack() {
			this.currentView = this.previousView;
		},
		toggleAddInstanceModal() {
			this.isAddInstanceModalOpen = !this.isAddInstanceModalOpen;
		},
		async loadInstances() {
			try {
				const rawResponse = await getInstances();

				// Validar completamente la respuesta usando el schema de Zod
				const response = validateBackendResponse(rawResponse);

				if (response.success && response.data) {
					if (isInstancesVecResponse(response.data)) {
						this.Instances = response.data.InstancesVec; // Formato tagged enum de Rust
						console.log(`Loaded ${this.Instances.length} instances`);
					} else {
						console.error("Expected InstancesVec data but got:", Object.keys(response.data)[0]);
					}
				} else if (!response.success && response.error) {
					console.error("Backend error:", response.error.error_type, response.error.error_message);
				}
			} catch (error) {
				if (error instanceof z.ZodError) {
					console.error("Response validation failed:", z.treeifyError(error));
				} else {
					console.error("Error loading instances:", error);
				}
			}
		}
		// async deleteInstance(instanceName: string) {
		//   this.Instances = this.Instances.filter(instance => instance.name !== instanceName)
		//   if (this.CurrentInstance?.name === instanceName) {
		//     this.CurrentInstance = null
		//   }
		//   await this.saveInstances()
		// }
	},
});
