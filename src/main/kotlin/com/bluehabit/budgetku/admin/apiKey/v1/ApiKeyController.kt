package com.bluehabit.budgetku.admin.apiKey.v1

import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping(
    value = ["/api/v1/admin"]
)
class ApiKeyController(
    private val apiKeyService: ApiKeyServiceImpl
) {
    @PostMapping(
        value = ["/api-key"],
        produces = ["application/json"]
    )
    fun generateToken(
    )=apiKeyService.generateApiKey()

    @DeleteMapping(
        value = ["/api-key/{api_key_id}"],
        produces = ["application/json"]
    )
    fun deleteCredential(
        @PathVariable("api_key_id") apiKeyId:Long
    )=apiKeyService.deleteApiKey(apiKeyId)
}